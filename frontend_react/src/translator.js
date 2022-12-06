export class Translator {
    translations = {}
    constructor(languagecode) {
        const url ="language/" + languagecode + ".json";
        console.log(url);
        fetch(url).then((res) => res.json()).then((json) => {this.translations = json; console.log(json)});
    }
}