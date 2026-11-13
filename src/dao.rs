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




        Ok(())
    }



    /*fn open_file(&self) -> Result<(), std::io::Error> {
        let file = File::open("data.txt")?;
        Ok(())
    }
*/
    /*ASSIGEMENT METHOD TO  MLTIDIMARRAY OF DATA AFTER SPITTING STRING SINGLE FIELD
    */

    fn split_string(stringJson: &str) -> [String;10] {
        let mut stringJson = stringJson.split(' ');
        stringJson[String;10]
    }

    /*METHOD OUTPUT INTERELY ARRAY DAO*/

}