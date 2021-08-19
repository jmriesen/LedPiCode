import {Light} from './components/light';

export class LightManager{
    private selected: Light[];
    constructor(private lights: Light[]){
        this.selected = [];
    }
    public select(light:Light){
        this.selected.push(light);
    }
    //public deselect(light:Light){}
    //this dose not work as I sercuvent the ui entirely.
    public set_color(color:string): Promise<void>{
        let promises = []
        this.selected.forEach(light => promises.push(light.set_color(color)));
        return Promise.all(promises);
    }
}
