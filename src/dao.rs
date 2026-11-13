use std::fs::File;
use std::io::{BufRead, BufReader};



struct StringDao{
    fields:  String,
}





impl StringDao{


    pub fn new(str: String) -> StringDao {
        StringDao{
            fields: str,
        }
    }

    /*OPEN FILE DAO IN RUST in buffering mode
    • Optimisation des performances : Lorsqu'un programme (en Python, C, etc.)
     ou le système d'exploitation ouvre et lit/écrit un fichier,
      il n'accède pas au disque (SSD/HDD) à chaque caractère ou petite
      ligne (ce qui serait très lent). La mémoire tampon : Les données
      transitent par un buffer en RAM. Par exemple,
       une commande write() copie d'abord les données dans
        le buffer ; le système se charge ensuite d'écrire
         le tout sur le disque en hidden lorsque
         le buffer est plein ou que le fichier est fermé.
         Instead of calling File::open_buffered(...),
         use BufReader::new(File::open(...)?):*/
    fn open()->std::io::Result<()>{

        let f= File::open("../assets/mots.json")?;
        let reader=BufReader::new(f);
        // reader.capacity() returns the internal buffer size
        // (default is 8 KB)

        assert!(reader.capacity()>0);

        /*Self::function(...)
        Calling an associated (static) function
        on the type
        Self::split_string(...)
        self.method(...)
        Calling a method on an instance*/
        let word = Self::split_string("testig string");
        println!("{:?}",word);
        Ok(())
    }



    /*fn open_file(&self) -> Result<(), std::io::Error> {
        let file = File::open("data.txt")?;
        Ok(())
    }
*/
    /*ASSIGEMENT METHOD TO  MLTIDIMARRAY OF DATA AFTER SPITTING STRING SINGLE FIELD
    */

    fn split_string(stringJson: &str) -> Vec<&str> {
        /*DDEFINED A ARRAY VARIABLR TYPE w TO OUTOUP*/
        let w: Vec<&str> = stringJson.split(' ').collect();
        // OUTPUT
        w
    }

    /*METHOD OUTPUT INTERELY ARRAY DAO*/

}



// testing Outside impl (at the bottom)
// Yes
// Standard Rust convention across all official crates an

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_only() {
        let word = StringDao::split_string("testig string");
        println!("{:?}", word);

        // Assert the expected outcome to ensure correctness automatically
        assert_eq!(word, vec!["testig", "string"]);
    }
}