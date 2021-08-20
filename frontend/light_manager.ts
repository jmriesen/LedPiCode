import {Light} from './components/light';

export class LightManager{
    private selected: Light[];
    constructor(private lights: Light[]){
        this.selected = [];
    }
    public select(light:Light){
        this.selected.push(light);
    }
    public deselect(light: Light){
        this.selected = this.selected.filter(entry => entry !== light);
    }
    //public deselect(light:Light){}
    //this dose not work as I sercuvent the ui entirely.
    public set_color(color:string): Promise<void>{
        let promises = []
        this.selected.forEach(light => promises.push(light.set_color(color)));
        return Promise.all(promises);
    }
    public get_lights(): Light[]{
        return this.lights;
    }
    public is_selected(light: Light):boolean{
        return this.selected.some(entry => light === entry);
    }
    public select_toggle(light: Light){
        if (this.is_selected(light)){
            this.deselect(light);
        }else{
            this.select(light);
        }
    }
    public static from_json(data:string[][]): LightManager{
        let lights:Light[] = [];
        data.forEach(light_str => lights.push(Light.from_data(light_str)));
        return new LightManager(lights);
    }

    public static from_backend(): Promise<LightManager>{
        return fetch('http://192.168.1.17:8000/status', {
            method: 'GET'
        })
            .then((response) =>response.json())
            .then((responseJson) => LightManager.from_json(responseJson ))
            .catch((error) => {
                console.error(error);
                return new LightManager([]);
            });

    }
}
