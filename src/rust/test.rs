use std::iter::*;
//use std::collections::*;

use crate::bot;
use crate::app_core::*;


pub fn test(){
    let my_bot = bot::Bot::new(Point{x:3,y:3},Direction::East);

   // my_bot.position.offset_by();
   let v = my_bot.manipulators.iter().map(|x|my_bot.position.offset_by(&x)).collect::<Vec<_>>();
   let aa = 1;
}