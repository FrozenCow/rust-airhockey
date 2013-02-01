extern mod sdl;
extern mod gl;
mod vec2;

use core::float::*;
use core::num::Num::*;
use core::ptr::ref_eq;
use core::cmp::Eq;
use core::vec::*;
use sdl::sdl::*;
use sdl::video::*;
use sdl::event::*;
use core::dvec::DVec;
use gl::gl::*;
//use option::{Some, None};
use vec2::*;


fn glVertex(v: Vec2) {
    glVertex2f(v.x as f32, v.y as f32);
}

trait GameObject {
    fn update(&mut self, game: &mut Game);
    fn draw(&self, game: &Game);
}

struct Paddle {
    position: Vec2,
    velocity: Vec2,
    radius: float
}
impl Paddle : GameObject {
    fn update(&mut self,game: &mut Game) {
        self.position += self.velocity;
        self.velocity *= 0.96;
    }
    fn draw(&self, game: &Game) {
        fillCircle(self.position, self.radius);
    }
}

struct Puck {
    position: Vec2,
    velocity: Vec2,
    radius: float
}
impl Puck: GameObject {
    fn update(&mut self,game: &mut Game) {
        self.position += self.velocity;
    }
    fn draw(&self, game: &Game) {
        fillCircle(self.position, self.radius);
    }
}

struct Game {
    objects: GameObjectManager,
    player: @mut Paddle,
    opponent: @mut Paddle,
    puck: @mut Puck,
    paddles: ~[@mut Paddle],
    field: Vec2,
    mouse: Vec2,
}

fn loadTexture() {
    let textureid:GLuint = 0;
    glGenTextures(1, &textureid);
    glBindTexture(GL_TEXTURE_2D, textureid);
    glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as GLfloat);
    glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as GLfloat);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLint); 
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as GLint);
}

struct GameObjectManager {
    objects: DVec<@GameObject>,
    pendingAdd: DVec<@GameObject>,
    pendingRemove: DVec<@GameObject>
}
fn GameObjectManager() -> GameObjectManager {
    GameObjectManager {
        objects: DVec(),
        pendingAdd: DVec(),
        pendingRemove: DVec()
    }
}
impl GameObjectManager {
    fn add(&self,gameObject: @GameObject) {
        self.pendingAdd.push(gameObject);
    }
    fn remove(&self,gameObject: @GameObject) {
        self.pendingRemove.push(gameObject);
    }
    fn handlePending(&self,) {
        while self.pendingAdd.len() > 0 {
            self.objects.push(self.pendingAdd.pop());
        }

        while self.pendingRemove.len() > 0 {
            remove_gameobject(&self.objects,self.pendingAdd.pop());
        }
    }
    pub fn each_mut(&self,f: fn(elem: &mut @GameObject) -> bool) {
        do self.objects.check_out |v| {
            let mut v:~[@GameObject] = move v;
            each_mut(v,f);
            self.objects.give_back(move v);
        }
    }
}
impl GameObjectManager: iter::BaseIter<@GameObject> {
    pure fn each(&self, blk: fn(v: &@GameObject) -> bool) { self.objects.each(blk) }
    pure fn size_hint(&self) -> Option<uint> { self.objects.size_hint() }
}
fn remove_gameobject(xs:&DVec<@GameObject>, x:@GameObject) -> Option<uint> {
    unsafe {
        do xs.check_out |v| {
            let mut v:~[@GameObject] = move v;
            let result = match position(v, |&y| { core::ptr::ref_eq(&x,&y) }) {
                None => None,
                Some(index) => {
                    v.remove(index);
                    Some(index)
                }
            };
            xs.give_back(move v);
            result
        }
    }
}



fn circle(position:Vec2, radius:float, f:fn(Vec2) -> bool) {
    let vertexCount = 20;
    int::range(0, vertexCount, |i| {
        let angle = (float::consts::pi*2./from_int(vertexCount)) * from_int(i);
        let p = Vec2 {x: float::cos(angle), y: float::sin(angle)};
        let v = p * radius + position;
        f(v)
    })
}

fn strokeCircle(position:Vec2, radius:float) {
    glBegin(GL_LINE_LOOP);
    for circle(position, radius) |v| {
        glVertex(v);
    }
    glEnd();
}

fn fillCircle(position: Vec2, radius: float) {
    glBegin(GL_TRIANGLE_FAN);
    for circle(position, radius) |v| {
        glVertex(v);
    }
    glEnd();
}

fn drawGame(game: &mut Game) {
    glClear(GL_COLOR_BUFFER_BIT);

    for game.objects.each |object| {
        object.draw(game);
    }

    strokeCircle(game.mouse, 10.0);

    swap_buffers();
}

fn updateGame(game:&mut Game) {
    for game.objects.each_mut |object| {
        object.update(game);
    }
}

fn handleControls(game:@mut Game) {
    let diff = game.mouse - game.player.position;
    let dist = diff.length();
    let maxSpeed = 50.;
    game.player.velocity = game.player.velocity * 0.09
        + diff.normalizeOrZero() * (if (dist < maxSpeed) { dist } else { maxSpeed }) * 0.91;
}

fn handleCollision(game:&mut Game) {
    for game.paddles.each |paddle| {
        let diff = (game.puck.position - paddle.position);
        if (diff.length() < game.puck.radius+paddle.radius) {
            game.puck.velocity -= getBounceImpact(diff.normalizeOrZero(), game.puck.velocity - paddle.velocity, 0.9);
        }
    };
    match getSurface(game.puck) {
        Some(surface) => {
            game.puck.velocity -= getBounceImpact(surface, game.puck.velocity, 0.9);
        }
        None => {}
    }
}

fn getBounceImpact(surface:Vec2, velocity:Vec2, bounciness:float) -> Vec2 {
    let impact = surface.dot(velocity);
    if (impact < 0.) {
        surface * impact * (1. + bounciness)
    } else {
        Vec2(0.,0.)
    }
}

fn getSurface(p:&Puck) -> Option<Vec2> {
    if p.position.x < p.radius { Some(Vec2(1.,0.)) }
    else if p.position.x > 640.-p.radius { Some(Vec2(-1.,0.)) }
    else if (p.position.y < p.radius) { Some(Vec2(0.,1.)) }
    else if (p.position.y > 480.-p.radius) { Some(Vec2(0.,-1.)) }
    else { None }
}

fn main() {
    init(&[InitEverything]);
    set_video_mode(640,480,32,&[],&[DoubleBuf,OpenGL]);

    // Initialize graphics
    glMatrixMode(GL_PROJECTION);
    glOrtho(0.0,640.0,480.0,0.0,0.0,1.0);
    glMatrixMode(GL_MODELVIEW);
    glLoadIdentity();

    let player: @mut Paddle = @mut Paddle {
        position: Vec2(100., 100.),
        velocity: Vec2(0.,0.),
        radius: 50.
    };

    let opponent: @mut Paddle = @mut Paddle {
        position: Vec2(540., 100.),
        velocity: Vec2(0.,0.),
        radius: 50.
    };

    let puck: @mut Puck = @mut Puck {
        position: Vec2{x:320.,y:240.},
        velocity: Zero,
        radius: 30.
    };

    let game = @mut Game {
        objects: GameObjectManager(),
        field: Vec2(640.,480.),
        mouse: Vec2(0.,0.),
        player: player,
        opponent: opponent,
        puck: puck,
        paddles: ~[player, opponent]
    };

    unsafe {
        game.objects.add(player as @GameObject);
        game.objects.add(puck as @GameObject);
        game.objects.add(opponent as @GameObject);

        game.objects.handlePending();
    
        let mut running = true;
        while running {
            loop {
                let event = poll_event();
                match event {
                    KeyDownEvent(k) => {
                        if (k.keycode == sdl::keyboard::SDLKEscape) {
                            running = false;
                        }
                        io::println(fmt!("%? %? %?", k.keycode, k.modifier, k.state));
                    }
                    KeyUpEvent(k) => {
                        io::println(fmt!("%? %? %?", k.keycode, k.modifier, k.state));
                    }
                    MouseMotionEvent(m) => {
                        game.mouse = Vec2(m.x as float, m.y as float);
                    }
                    QuitEvent => {
                        running = false;
                    }
                    NoEvent => { break; }
                }
            }
            handleControls(game);
            updateGame(game);
            handleCollision(game);
            drawGame(game);
            game.objects.handlePending();
        };
    }
    quit();
}