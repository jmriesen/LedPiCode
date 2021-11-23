import 'jest';
import {Light, black,white} from './light';

global.fetch = require('jest-fetch-mock');

beforeAll(() => {
    //TODO mocking needs to be added
    Light.base_url = 'http://localhost:8000/';
});

beforeEach(() => {
    global.fetch.resetMocks();
});

test('should be able to construct light from backend data',() =>{
    let light_data=  [
        "light1",
        "#000000ff"
    ]
    let light = Light.from_data(light_data);
    expect(light.get_name()).toBe("light1");
    expect(light.get_color()).toBe(black);
})
test('should send a message to the backend when color is set', async () =>{
    let light = new Light("name",black);
    expect(light.get_color()).toBe(black);
    await light.set_color(white);
    expect(light.get_color()).toBe(white);
})

test('black should toggling to white.', async () =>{
    let light = new Light("name",black);
    await light.toggle();
    expect(light.get_color()).toBe(white);
})


test('non black should toggling to black.', async () =>{
    let light = new Light("name",white);
    await light.toggle();
    expect(light.get_color()).toBe(black);

    await light.set_color("#ff0000ff");
    await light.toggle();
    expect(light.get_color()).toBe(black);
})
