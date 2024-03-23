// Typewritter simulation
// Peter, 23/03/2024

use std::io::{self, Write};
use std::{thread, time};
use std::process::Command;

fn typewritter(text: &str) {
    for i in text.chars() {
        print!("{}", i);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(30));

        if i == '.' { 
            thread::sleep(time::Duration::from_millis(2000));
            println!();
        }
    }
}

fn main() {
    Command::new("clear")
        .status()
        .unwrap();

    let paragrafos = vec![
        "No coração da cidade, onde arranha-céus se elevam em direção ao céu, e luzes cintilantes pintam o cenário noturno, existe uma energia palpável que pulsa pelas ruas pavimentadas. É aqui que o ritmo frenético da vida urbana encontra a calma serena dos parques verdejantes e dos cafés acolhedores. Cada esquina conta uma história, cada edifício uma narrativa, e cada rosto na multidão uma expressão de sonhos e aspirações.",
        "Sob a vastidão do céu estrelado, as montanhas se erguem majestosas, testemunhas silenciosas do tempo que passa. No topo, o ar rarefeito acaricia os pulmões, enquanto a beleza selvagem da natureza envolve os sentidos. Cachoeiras rugem em cascata, alimentando riachos que serpenteiam pelas encostas cobertas de vegetação exuberante. É aqui, entre picos nevados e vales verdejantes, que a alma encontra paz e renovação.",
        "Nos confins do espaço sideral, além das fronteiras conhecidas da galáxia, mundos desconhecidos aguardam a exploração destemida da humanidade. Naves espaciais cortam o vazio negro do cosmos, guiadas pela luz das estrelas distantes. Planetas exóticos com paisagens alienígenas oferecem mistérios a serem desvendados, enquanto civilizações além da imaginação habitam sistemas solares distantes. É um universo de possibilidades infinitas, onde o desconhecido é apenas o começo de uma jornada épica.",
        "Nas profundezas do oceano, onde a luz do sol mal alcança, um mundo misterioso espera para ser descoberto. Recifes de coral brilham com cores vibrantes, abrigando uma miríade de criaturas marinhas que dançam ao ritmo das correntes oceânicas. Navios naufragados contam histórias de tempos passados, enquanto criaturas lendárias espreitam nas sombras submarinas. É um reino de maravilhas subaquáticas, onde a aventura mergulha nas profundezas desconhecidas.",
        "Nos confins da mente humana, onde os pensamentos fluem como rios de consciência, e os sonhos tecem tapeçarias de imaginação, existe um vasto e infinito universo interior. Ideias se entrelaçam em um emaranhado de conexões neurais, criando paisagens mentais únicas e inexploradas. É aqui que a criatividade floresce, onde a introspecção revela verdades ocultas e a imaginação é livre para vagar por territórios desconhecidos. É um mundo de possibilidades ilimitadas, onde a mente humana é tanto exploradora quanto criadora, moldando a realidade com cada pensamento e cada inspiração."
    ];

    for paragrafo in paragrafos {
        typewritter(&paragrafo);
        println!();
    }
}
