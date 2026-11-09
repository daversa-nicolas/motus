import init, {Lettres} from './pkg/product.js';


async function run() {
    await init();
}


export function handleClickProduct() {


    /* USING STOCK ARRAY TS FOR LETTRES INPUT
    TO OUTPUT IN WASM
    that WILL BE USED IN RUST
    IN listLetters: Vec<String> DUMMY ARGUMENT*/
    let arrayLettre: string[] = new Array();

    /*ARRAY ONLY FOR INPUT*/
    const input1 = document.getElementById("inputCaseA");
    const input2 = document.getElementById("inputCaseB");
    const input3 = document.getElementById("inputCaseC");
    const input4 = document.getElementById("inputCaseD");
    const input5 = document.getElementById("inputCaseE");
    const input6 = document.getElementById("inputCaseF");

    /*IN TS NEED CHECK NO NULL ELEMENT*/
    if (input1 instanceof HTMLInputElement &&
        input2 instanceof HTMLInputElement &&
        input3 instanceof HTMLInputElement &&
        input4 instanceof HTMLInputElement &&
        input5 instanceof HTMLInputElement &&
        input6 instanceof HTMLInputElement
    ) {
        arrayLettre[0] = input1.value;
        arrayLettre[1] = input2.value;
        arrayLettre[2] = input3.value;
        arrayLettre[3] = input4.value;
        arrayLettre[4] = input5.value;
        arrayLettre[5] = input6.value;
    }

    console.log("dim array" + arrayLettre.length)
    arrayLettre.push("1 testing");
    console.log("firstr elemetn array : " + arrayLettre[0]);
    console.log("second element array : " + arrayLettre[1]);
    console.log(arrayLettre[2]);
    console.log(arrayLettre[3]);
    console.log(arrayLettre[4]);

    /*instanced class Lettre to acces methods*/
    /*TRY DUMMY ARRAY ? BETTER SINGLE FIELD MAYBE*/
    /*THIS IS IMPORTED CLASS FROM RUST TO TS*/
    const lettreObject = new Lettres("Atest", "Btest", arrayLettre);

   /*IN typescript impossible videly elemelent so testing before*/
   const output1= lettreObject.get_array_rust()[0];
   const output2= lettreObject.get_array_rust()[1];
   console.log("THIS IS OUTPUT 1 FROM WASM ARRAY : "+output1);
   console.log("THIS IS OUTPUT 2 FROM WASM ARRAY : "+output2);
   const element1 = document.getElementById("outputts1");
   const element2 = document.getElementById("outputts2");
   // OUTPUT TESTING of array
   if (output1 && output2 && element1 instanceof HTMLElement
       && element2 instanceof HTMLElement) {
       element1.innerText = lettreObject.get_array_rust()[0];
       element2.innerText = lettreObject.get_array_rust()[1];
   }


   /*// in typescript impossible videly elemelent so testing before
    // OUTPUT HERE in label
    const outpu1 = document.getElementById("outputts");
    const outpu2 = document.getElementById("outputts2");

    if (outpu1 && outpu2) {
        outpu1.innerText = lettreObject.get_lettre();
        outpu2.innerText = lettreObject.get_lettre2();
    }*/
    /* document.getElementById("outputts").innerText =
         lettreObject.get_lettre();*/


}

/*IN TS NO POSSIBLE VALUE NULL TO ASSIGNEMENT , BEFORE NEED TO CHECK IF ELEMENT EXIST IN THIS :*/
const calculateInput = document.getElementById("calculateinput");
/*export clickproduct js function must be different to real name product called in scope*/
if (calculateInput !== null) {
    calculateInput.addEventListener("click", handleClickProduct);
}

run();