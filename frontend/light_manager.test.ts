import 'jest';
import {Light, black, white} from './components/light';
import {LightManager} from './light_manager';

beforeAll(() => {
    //TODO mocking needs to be added
    Light.base_url = 'http://localhost:8000/';
});

test('should update selected lights', async ()=>{
    let light_1 = new Light("light1",black);
    await light_1.set_color(black);
    let light_2 = new Light("light2",black);
    await light_2.set_color(black);
    let manager = new LightManager([light_1,light_2]);
    manager.select(light_1);
    await manager.set_color(white);
    expect(light_1.get_color()).toBe(white);
    expect(light_2.get_color()).toBe(black);
})
