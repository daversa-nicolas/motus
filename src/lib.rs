mod dao;
use wasm_bindgen::prelude::*;
use std::clone::Clone;

/*
Le problème vient de wasm-bindgen : [String; 126]
 n'est pas un type directement pris en charge pour
  un champ d'une structure exportée avec #[wasm_bindgen].
Tu peux conserver ton tableau en Rust, mais -->> il faut distinguer
 les champs internes des champs exposés à JavaScript.*/

// avec wasm-binggen les champs sont expose'

pub struct LettresInner {
    lettre: String,
    lettre2: String,
    /*If you specifically need a fixed-size [String; 126],
     you can keep it in a separate, non-exported Rust struct
     and expose only methods through wasm-bindgen.*/
    array_rust: [String; 6],
}

#[wasm_bindgen]
pub struct Lettres{
    inner: LettresInner,
}

/* l'erreur expected [String; 126],
but found () ne correspond pas au comportement normal de std::array::from_fn.
 Et puisque cargo check réussit, il est très probable que RustRover
 affiche une erreur d'analyse incorrecte ou obsolète.*/

// IMPLEMENTING METHODS OF CLASS LETTRE
#[wasm_bindgen]
impl Lettres {
    /*So FROM WASM THE DUMMY ARGUMENT ARRAY MAYBE A
    WASM ARRAY Vec<> as ArrayList mutable
    */
    #[wasm_bindgen(constructor)]
    pub fn new(lettre: String, lettre2: String, list_letters: Vec<String>) -> Lettres {
        /*the field list_letters: [String; 6] must be initialized with a fixed-size array of Strings */
        /*Option A: Using std::array::from_fn (Safe with any vector length)
        Takes items from the vector, filling missing elements with empty strings:
        let mut iter = list_letters.into_iter();
        let array: [String; 6] = std::array::from_fn(|_| iter.next().unwrap_or_default()); */

        /*Option B: Using try_into() (When the vector is guaranteed or expected to be length 6)*/
        /*list letters is an array from WASM : need maybe to initialised in Rust
        array arra_rust as fields of class Lettres*/
        let array_rust: [String; 6] = list_letters
            .try_into()
            .unwrap_or_else(|_| std::array::from_fn(|_| String::new()));

        /*2. Initialize array with Default / Empty Values
        If array is an internal buffer initialized to empty strings regardless of list_letters:*/

        Lettres {
            inner: LettresInner {
                lettre,
                lettre2,
                array_rust,
            },
        }
    }


    pub fn get_lettre2(&self) -> String {

        self.inner.lettre2.clone()
    }

    // comment TODO
    pub fn get_lettre(&self) -> String {
        println!("get lettre");

        self.inner.lettre.clone()
    }

    /* TODO AN ANTERNAL ARRAY RUST ARRAYC FOR CALCULE OF DATA FOUNDED IN
     DAO */

    /* OUTPUT OF ARRAY TESTED WITHOUT ERROR */
    /*Option A (Vec<String> with .to_vec()) whenever
     you want JavaScript/Wasm to receive the full array.*/
    pub fn get_array_rust(& self) -> Vec<String>{
        println!("get list_letters");
        println!("ARRAY RUST {:?}", self.inner.array_rust);
        self.inner.array_rust.to_vec()
    }




}


#[wasm_bindgen]
pub fn hello() -> String {
    "Hello, world!".to_string()
}
