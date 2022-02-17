#[derive(Debug)]
enum JapaneseDogBreeds {
    AkitaKen,
    HokkaidoInu,
    KaiKen,
    KishuInu,
    ShibaInu,
    ShikokuKen,
}

impl From<u32> for JapaneseDogBreeds {
    fn from(other: u32) -> Self {
        match other {
            other if JapaneseDogBreeds::AkitaKen as u32 == other => {
                JapaneseDogBreeds::AkitaKen
            }
            other if JapaneseDogBreeds::HokkaidoInu as u32 == other => {
                JapaneseDogBreeds::HokkaidoInu
            }
            other if JapaneseDogBreeds::KaiKen as u32 == other => {
                JapaneseDogBreeds::KaiKen
            }
            other if JapaneseDogBreeds::KishuInu as u32 == other => {
                JapaneseDogBreeds::KishuInu
            }
            other if JapaneseDogBreeds::ShibaInu as u32 == other => {
                JapaneseDogBreeds::ShibaInu
            }
            other if JapaneseDogBreeds::ShikokuKen as u32 == other => {
                JapaneseDogBreeds::ShikokuKen
            }
            _ => panic!("Unknown breed!"),
        }
    }
}

fn main() {
    println!("{:?}", JapaneseDogBreeds::ShibaInu);
    println!("{:?}", JapaneseDogBreeds::ShibaInu as u32);
    println!("{:?}", JapaneseDogBreeds::from(4));
}
