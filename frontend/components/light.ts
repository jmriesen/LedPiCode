//import fetch from "node-fetch";
const fetch = require("node-fetch");
export const black = "#000000ff";
export const white = "#ffffffff";

export class Light{
    public static base_url :string = 'http://192.168.1.17:8000/'
    constructor(private name:string,private color:string){
    }
    public get_name():string{
        return this.name;
    }
    public get_color():string{
        return this.color;
    }
    public set_color(color:string):Promise<void>{
        var url = Light.base_url+'set/'+this.name;
        return fetch(url,
                     {
            method: 'Post',
            body: JSON.stringify(
                color
            )}
                    )
            .then((response) =>{
                return response.json();
            })
            .then((responseJson) => {
                this.color = responseJson[1];
            })
            .catch((error) => {
                console.error(error);
            })
    }
    public toggle():Promise<void>{
        if (this.color == black){
            return this.set_color(white);
        }else{
            return this.set_color(black);
        }
    }
    public static from_data(data:string[]):Light{
        return new Light(data[0],data[1]);

    }
}

