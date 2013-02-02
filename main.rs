extern mod sdl;
mod gl;
mod vec2;
mod pendinglist;

use core::float::*;
use core::num::Num::*;
use core::ptr::ref_eq;
use core::cmp::Eq;
use core::vec::*;
use sdl::sdl::*;
use sdl::video::*;
use sdl::event::*;
use gl::*;
use pendinglist::*;
//use option::{Some, None};
use vec2::*;


unsafe fn glVertex(v: Vec2) {
    glVertex2f(v.x as f32, v.y as f32);
}

pub trait GameObject {
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
        // Limit velocity of puck
        let speed = self.velocity.length();
        let direction = self.velocity.normalizeOrZero();
        self.velocity = direction * if speed > 30. { 30. } else { speed };

        // Apply velocity
        self.position += self.velocity;

        // Apply damping
        self.velocity *= 0.99;

    }
    fn draw(&self, game: &Game) {
        fillCircle(self.position, self.radius);
    }
}

struct Game {
    objects: PendingList<GameObject>,
    player: @mut Paddle,
    playerScore: uint,
    opponent: @mut Paddle,
    opponentScore: uint,
    puck: @mut Puck,
    paddles: ~[@mut Paddle],
    field: Vec2,
    goalSize: float,
    mouse: Vec2
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
    unsafe {
        glBegin(GL_LINE_LOOP);
        for circle(position, radius) |v| {
            glVertex(v);
        }
        glEnd();
    }
}

fn fillCircle(position: Vec2, radius: float) {
    unsafe {
        glBegin(GL_TRIANGLE_FAN);
        for circle(position, radius) |v| {
            glVertex(v);
        }
        glEnd();
    }
}

fn drawGame(game: &Game) {
    unsafe { glClear(GL_COLOR_BUFFER_BIT); }

    for game.objects.each |object| {
        object.draw(game);
    }

    strokeCircle(game.mouse, 10.0);

    drawScore(game.playerScore, Vec2(10.,10.), Vec2(10.,0.));
    drawScore(game.opponentScore, Vec2(game.field.x-10.,10.), Vec2(-10.,0.));

    swap_buffers();
}

fn drawScore(score: uint, position: Vec2, direction: Vec2) {
    let columns = float::sqrt(score as float) as uint;
    for core::uint::range(0,score) |index| {
        let x = index / columns;
        let y = index % columns;
        let right = direction;
        let down = Vec2(0.,1.)*direction.length();
        fillCircle(position+right*(x as float)+down*(y as float), 5.);
    }
}

fn updateGame(game:&mut Game) {
    for game.objects.each_mut |object| {
        object.update(game);
    }
}

fn handleControls(game:&mut Game) {
    let diff = game.mouse - game.player.position;
    let dist = diff.length();
    let maxSpeed = 50.;
    game.player.velocity = game.player.velocity * 0.3
        + diff.normalizeOrZero() * (if (dist < maxSpeed) { dist } else { maxSpeed }) * 0.5;
}

fn handleCollision(game:&mut Game) {
    // Handle paddle - puck collision
    for game.paddles.each |paddle| {
        let diff = (game.puck.position - paddle.position);
        if (diff.length() < game.puck.radius+paddle.radius) {
            game.puck.velocity -= getBounceImpact(diff.normalizeOrZero(), game.puck.velocity - paddle.velocity, 0.9);
        }
    };
    // Handle field boundaries - puck collision
    match getSurface(game, game.puck) {
        Some(surface) => {
            game.puck.velocity -= getBounceImpact(surface, game.puck.velocity, 0.9);
        }
        None => {}
    }
}

fn between<T:Ord>(x:T, a:T, b:T) -> bool { x > a && x < b }
fn distance(a:Vec2,b:Vec2) -> float { (a-b).length() }

fn handleOpponent(game:&mut Game) {
    let defenceSpeed = 3.;
    let attackSpeed = 10.;
    let position = game.opponent.position;
    let goal = Vec2(game.field.x, game.field.y*0.5);
    let puck = game.puck;
    let goalDirection = (position - goal).normalizeOrZero();
    let puckDirection = (position - puck.position).normalizeOrZero();
    let puckDistance = (position - puck.position).length();
    let desiredVelocity = if // Should we dash forward (attack) ?
           goalDirection.dot(puckDirection) < 0.
        && puckDistance < 100.
        && between(puckDirection.dot(puck.velocity), -2.,10.)
    { velocityTowards(position, puck.position, attackSpeed) }
    else if // Should we move towards the puck (player is too far away) ?
           goalDirection.dot(puckDirection) < 0.
        && puck.velocity.length() < 3.
        && distance(game.player.position,puck.position) / distance(position,puck.position) > 2.0
    { velocityTowards(position, puck.position, defenceSpeed) }
    else // Should we stand between puck and goal (defend) ?
    { velocityTowards(position, (puck.position + goal) * 0.5, defenceSpeed) };

    game.opponent.velocity = game.opponent.velocity * 0.80 + desiredVelocity * 0.15;
}

fn velocityTowards(source:Vec2, destination:Vec2, speed:float) -> Vec2 {
    let diff = destination - source;
    let distance = diff.length();
    let direction = diff.normalizeOrZero();
    if distance < speed { direction * distance }
    else { direction * speed }
}

fn handleGoals(game:&mut Game) {
    let p = game.puck;
    if p.position.y > game.field.y*0.5-game.goalSize*0.5 && p.position.y < game.field.y*0.5+game.goalSize*0.5 {
        if p.position.x < 0. && p.velocity.x < 0. {
            p.position = game.field*0.5;
            p.velocity = Zero;
            game.opponentScore = game.opponentScore + 1;
        } else if p.position.x > game.field.x && p.velocity.x > 0. {
            p.position = game.field*0.5;
            p.velocity = Zero;
            game.playerScore = game.playerScore + 1;
        }
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

fn getSurface(game: &Game, p:&Puck) -> Option<Vec2> {
    // Handle goals (part of the boundaries where collision is disabled)
    if p.position.y > game.field.y*0.5-game.goalSize*0.5 && p.position.y < game.field.y*0.5+game.goalSize*0.5 { None }
    // Handle walls
    else if p.position.x < p.radius { Some(Vec2(1.,0.)) }
    else if p.position.x > game.field.x-p.radius { Some(Vec2(-1.,0.)) }
    else if (p.position.y < p.radius) { Some(Vec2(0.,1.)) }
    else if (p.position.y > game.field.y-p.radius) { Some(Vec2(0.,-1.)) }
    // Nothing else
    else { None }
}

fn setupGame() -> ~mut Game {
    let field = Vec2(640.,480.);

    let player: @mut Paddle = @mut Paddle {
        position: Vec2(100., field.y*0.5),
        velocity: Vec2(0.,0.),
        radius: 40.
    };

    let opponent: @mut Paddle = @mut Paddle {
        position: Vec2(field.x-100., field.y*0.5),
        velocity: Vec2(0.,0.),
        radius: 40.
    };

    let puck: @mut Puck = @mut Puck {
        position: Vec2{x:320.,y:240.},
        velocity: Zero,
        radius: 30.
    };

    let goalSize = 250.;

    let game = ~mut Game {
        objects: PendingList(),
        field: Vec2(640.,480.),
        goalSize: goalSize,
        mouse: Vec2(0.,0.),
        player: player,
        playerScore: 0,
        opponent: opponent,
        opponentScore: 0,
        puck: puck,
        paddles: ~[
            player,
            opponent,
            @mut Paddle { position: Vec2(0., 480.*0.5-goalSize*0.5), velocity: Zero, radius: 20. },
            @mut Paddle { position: Vec2(0., 480.*0.5+goalSize*0.5), velocity: Zero, radius: 20. },
            @mut Paddle { position: Vec2(640., 480.*0.5-goalSize*0.5), velocity: Zero, radius: 20. },
            @mut Paddle { position: Vec2(640., 480.*0.5+goalSize*0.5), velocity: Zero, radius: 20. }
        ]
    };

    addPaddles(game);
    game.objects.add(puck as @GameObject);
    game.objects.handlePending();

    move game
}

fn addPaddles(game:&mut Game) {
    for game.paddles.each |&paddle| {
        game.objects.add(paddle as @GameObject);
    }
}

fn gameLoop(game: &mut Game, update: fn(&mut Game) -> bool) {
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
        running = running && update(game);
    }
}

fn main() {
    init(&[InitEverything]);
    set_video_mode(640,480,32,&[],&[DoubleBuf,OpenGL]);

    unsafe {
        // Initialize graphics
        glMatrixMode(GL_PROJECTION);
        glOrtho(0.0,640.0,480.0,0.0,0.0,1.0);
        glMatrixMode(GL_MODELVIEW);
        glLoadIdentity();
    }

    let game = setupGame();

    for gameLoop(game) |game|{
        handleControls(game);
        updateGame(game);
        handleOpponent(game);
        handleCollision(game);
        handleGoals(game);
        drawGame(game);
        game.objects.handlePending();
    };
    quit();
}