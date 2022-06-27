use std::env::var;
use fermium::events::{SDL_Event, SDL_EventType, SDL_PollEvent, SDL_KEYDOWN, SDL_QUIT};
use fermium::keycode::SDLK_q;
use fermium::prelude::{
    SDL_CreateRenderer, SDL_RenderClear, SDL_RenderDrawRect, SDL_SetRenderDrawColor,
};
use fermium::rect::SDL_Rect;
use fermium::renderer::{
    SDL_RenderFillRect, SDL_RenderPresent, SDL_Renderer, SDL_RENDERER_ACCELERATED,
};
use fermium::video::{
    SDL_CreateWindow, SDL_DestroyWindow, SDL_Window, SDL_WindowFlags, SDL_WINDOWPOS_CENTERED,
};
use fermium::{c_char, c_int, SDL_Quit, SDL_INIT_VIDEO};
use fermium::timer::SDL_Delay;

const title: *const c_char = "hello".as_ptr().cast();

#[derive(Clone)]
pub struct Game {
    window: *mut SDL_Window,
    renderer: *mut SDL_Renderer,
    pub quit: bool
}

impl Game {
    pub unsafe fn new() -> Self {
        fermium::SDL_Init(SDL_INIT_VIDEO);
        let window = SDL_CreateWindow(
            title,
            SDL_WINDOWPOS_CENTERED,
            SDL_WINDOWPOS_CENTERED,
            64 * 5,
            32 * 5,
            0,
        );
        return Game {
            window,
            renderer: SDL_CreateRenderer(window, -1 as c_int, SDL_RENDERER_ACCELERATED.0),
            quit:false
        };
    }
    pub unsafe fn init(&self) {
        SDL_SetRenderDrawColor(self.renderer, 0, 0, 0, 255);
        SDL_RenderClear(self.renderer);
    }
    unsafe fn add_rect(&self, x: c_int, y: c_int) {
        // Creat a rect at pos ( 50, 50 ) that's 50 pixels wide and 50 pixels high.
        let r = SDL_Rect { x, y, w: 5, h: 5 };

        // Set render color to blue ( rect will be rendered in this color )
        SDL_SetRenderDrawColor(self.renderer, 0, 0, 255, 255);

        // Render rect
        SDL_RenderFillRect(self.renderer, &r);
    }
    pub unsafe fn draw(&self, &display: &[[bool; 32]; 64]) {
        for x in 0..display.len() {
            for y in 0..display[0].len() {
                if display[x][y] {
                    self.add_rect(x as i32 * 5, y as i32 *5);
                }
            }
        }
    }

    pub unsafe fn commit(&self) {
        SDL_RenderPresent(self.renderer);
    }

    pub unsafe fn run(&mut self){
            let mut event: SDL_Event = SDL_Event::default();
            while SDL_PollEvent(&mut event) != 0 {
                match event.type_ {
                    SDL_KEYDOWN => match event.key.keysym.sym {
                        K => {}
                    },
                    _ => {}
                }
            }
        if(self.quit) {
            SDL_DestroyWindow(self.window);
            SDL_Quit();
        }
        return;
    }
}
