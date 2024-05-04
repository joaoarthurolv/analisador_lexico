struct Analisador {
    entrada: String,
    posição: usize,
}

impl Analisador {
   fn novo(entrada: String) -> Self {
        Self {
            entrada,
            posição: 0,
        }
   }
   
   fn próximo(&mut self) -> (bool, usize, String) {
       // Se chegar ao fim da String
       if self.posição >= self.entrada.len() {
            return (false, self.posição, String::new());
        }
        
        let mut pos = self.posição;
        let mut numero = String::new();
        
        while pos < self.entrada.len() {
            let c = self.entrada.chars().nth(pos).unwrap();
            
            match c {
                '+' | '-' => {
                    println!("({}, {})", pos, c);
                    pos += 1;
                }
                '0'..='9' => {
                    let mut aux = c;
                    loop {
                        let next = self.entrada.chars().nth(pos + 1).unwrap();
                    
                        numero.push(aux);
                        pos += 1;
                        
                        if next.is_whitespace() || next == '\0' {
                            break;
                        }
                        aux = next;
                    }
                    
                    println!("({}, {})", pos, numero);
                }
                
                _ => {
                    pos += 1;
                    continue;
                }
            }
        }

        self.posição = pos;
        (false, pos, String::new())
   }
   /*
   fn devolver(&mut self, pos: usize, s: String) {
       todo!()
   }*/
}

fn main() {
    let string_analisada = String::from("97 + 7");
    let mut analisador = Analisador::novo(string_analisada.clone());
    
     
    loop {
        let (encontrado, pos, resultado) = analisador.próximo();
        
        if pos >= string_analisada.len() {
            break;
        }
    } 
}