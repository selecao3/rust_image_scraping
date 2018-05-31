extern crate reqwest;
extern crate rand;
extern crate scraper;

use reqwest::get;
use scraper::{Html,Selector};
//use image::GenericImage;
//use std::fs::*;
//use std::path::Path;
use std::io;
use std::io::Read;
use std::fs;
use std::fs::File;
use std::io::Write;
use rand::*;

fn main() {
    for i in 1..100{
        //let url = get(format!("http://moeimg.net/page/{}", i)).unwrap().text().ok().unwrap();
        let url = get(format!("http://moeimg.net/page/{}", i).trim_right()).unwrap().text().unwrap();


        let hoge = Html::parse_fragment(url.trim_right());
        let selector = Selector::parse("a.more-link").unwrap();
        for element in hoge.select(&selector){

            let mut aaa = element.value().attr("href").unwrap();
            let img_page = get(aaa).unwrap().text().ok().unwrap();
            let hage = Html::parse_fragment(img_page.trim_right());
            let img = Selector::parse("img.thumbnail_image").unwrap();

            for element2 in hage.select(&img){
                let mut bbb = element2.value().attr("src").unwrap();

                let mut img_url = get(bbb).unwrap();

                let mut body:Vec<u8> = vec![];
                img_url.read_to_end(&mut body).unwrap();
                let number:i32 = random();
                fs::create_dir("./img-hoge").ok();


                let mut f = File::create(format!("./img-hoge/hoge{}.jpg",number)).unwrap();
                f.write_all(&body);
            }

            println!("{}",aaa);
        }
    }
}

