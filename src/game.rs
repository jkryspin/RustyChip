use fermium::{c_char, c_int, SDL_INIT_VIDEO, SDL_Quit};
use fermium::events::{SDL_Event, SDL_EventType, SDL_KEYDOWN, SDL_PollEvent, SDL_QUIT};
use fermium::keycode::SDLK_q;
use fermium::prelude::{SDL_CreateRenderer, SDL_RenderClear, SDL_RenderDrawRect, SDL_SetRenderDrawColor};
use fermium::rect::SDL_Rect;
use fermium::renderer::{SDL_Renderer, SDL_RENDERER_ACCELERATED, SDL_RenderFillRect, SDL_RenderPresent};
use fermium::video::{SDL_CreateWindow, SDL_DestroyWindow, SDL_WindowFlags, SDL_WINDOWPOS_CENTERED};
const title: *const c_char = "hello".as_ptr().cast();


struct Game {
    renderer: SDL_Renderer
}

impl Game{
    pub fn new() -> Self {
        Game {
            renderer: SDL_Renderer::default(),
        }
    }

    pub unsafe fn setup_renderer(&mut self){
        let window = SDL_CreateWindow(title, SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, 64*4, 32*4, 0);
        self.renderer= *SDL_CreateRenderer(window, -1 as c_int, SDL_RENDERER_ACCELERATED.0);
    }

}

pub unsafe fn main() {
    fermium::SDL_Init(SDL_INIT_VIDEO);


    SDL_RenderClear(renderer);

    // Creat a rect at pos ( 50, 50 ) that's 50 pixels wide and 50 pixels high.
    let r = SDL_Rect{
        x:50,
        y: 50,
        w: 50,
        h: 50
    };

    // Set render color to blue ( rect will be rendered in this color )
    SDL_SetRenderDrawColor( renderer, 0, 0, 255, 255 );

    // Render rect
    SDL_RenderFillRect( renderer, &r );

    // Render the rect to the screen
    SDL_RenderPresent(renderer);

    let mut quit = false;
    while !&quit{
        let mut event: SDL_Event = SDL_Event::default();
        while SDL_PollEvent(&mut event)!=0 {
            match event.type_ {
                SDL_KEYDOWN => match event.key.keysym.sym {
                    SDLK_q => quit = true,
                    _ => {}
                }
                _ => {}
            }
        }
    }
    SDL_DestroyWindow(window);
    SDL_Quit();
    return;
}