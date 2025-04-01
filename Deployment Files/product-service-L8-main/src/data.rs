use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product { 
            id: 1,
            name: "Insignia 50 Class 4K UHDTV (2160p)".to_string(),
            price: 9.99,
            description: "The Insignia 50 Class 4K UHDTV (2160p) HDR Smart LED-LCD TV (NS-50F301NA24) offers a  49.5 screen size with 3840 x 2160 4K UHD resolution , perfect for viewing at 6-10 feet".to_string(),
            image: "/1.webp".to_string()
        },
        Product {
            id: 2,
            name: "2Pack Metal Wall Mount for eufy SoloCam ".to_string(),
            price: 6.99,
            description: "Compatible with eufyCam 2C/2C Pro, 2/2 Pro, 3C/S300, 3/S330, Eufycam E, also for SoloCam S220/C210, also compatible with other cameras with 1/4 threaded hole".to_string(),
            image: "/2.webp".to_string()
        },
        Product {
            id: 3,
            name: "Sandisk SSD Extreme 1TB Extreme".to_string(),
            price: 12.99,
            description: "​The SanDisk Extreme Portable SSD is a high-performance external solid-state drive designed to meet the demands of users requiring fast and reliable storage".to_string(),
            image: "/3.webp".to_string()
        },
        Product {
            id: 4,
            name: "Apple - Mac mini Desktop".to_string(),
            price: 11.99,
            description: "​The Apple Mac mini is a compact desktop computer that delivers powerful performance in a small form factor".to_string(),
            image: "/4.webp".to_string()
        },
        Product {
            id: 5,
            name: "Apple Watch Ultra 2".to_string(),
            price: 8.99,
            description: "​The Apple Watch Ultra 2, released in September 2023, is a premium smartwatch designed for sports and adventure enthusiasts".to_string(),
            image: "/5.webp".to_string()
        },
        Product {
            id: 6,
            name: "Apple AirPods 4 Wireless Earbuds".to_string(),
            price: 14.99,
            description: "The Apple AirPods 4, introduced in September 2024, represent a significant advancement in Apple's wireless earbud lineup, offering enhanced features and improved performance".to_string(),
            image: "/6.webp".to_string()
        },
        Product {
            id: 7,
            name: "SAMSUNG Q-Series 11.1.4ch Wireless".to_string(),
            price: 19.99,
            description: "​The Samsung Q-Series 11.1.4 Channel Wireless Dolby Atmos Soundbar, particularly the HW-Q990D model, offers an immersive audio experience designed to elevate home entertainment systems".to_string(),
            image: "/7.webp".to_string()
        },
        Product {
            id: 8,
            name: "Roku Streaming Stick 4K 2022".to_string(),
            price: 7.99,
            description: "​The Roku Streaming Stick 4K, released in 2022, is a compact and powerful streaming device that enhances your TV's capabilities by providing access to a wide range of streaming services in high-quality resolution".to_string(),
            image: "/8.webp".to_string()
        },
        Product {
            id: 9,
            name: "Apple MacBook Air 13-inch 2022 M2".to_string(),
            price: 3.99,
            description: "​The Apple MacBook Air 13-inch (2022) with M2 chip is a sleek and powerful ultrabook designed for users seeking portability without sacrificing performance".to_string(),
            image: "/9.webp".to_string()
        },
        Product {
            id: 10,
            name: "Apple 2022 10.9-inch iPad ".to_string(),
            price: 5.99,
            description: "​The Apple iPad (10th Generation), released in October 2022, offers a blend of performance and portability, making it suitable for both work and entertainment".to_string(),
            image: "/10.webp".to_string()
        }
    ]
}