import { StatusBar } from 'expo-status-bar';
import React, {Component, } from 'react';
import { KeyboardAvoidingView, FlatList, StyleSheet, Text, View, TextInput, TouchableOpacity, Keyboard, ScrollView } from 'react-native';
import { ColorPicker,fromHsv,} from 'react-native-color-picker';
import Light_Ui from './components/light_ui';
import {LightManager} from './light_manager';

export default class App extends Component {
    state = {
        manager: new LightManager([]),
    }
    constructor(props:any) {
        super(props);
        LightManager.from_backend()
        .then(new_manager => this.setState({manager:new_manager}));
    }
    render() {
        return (
            <View style={styles.container}>
                <ColorPicker
                    onColorSelected={
                    (color)=>{
                        this.state.manager.set_color(color+"ff")
                            .then( temp => this.setState({}));
                    }
                    }
            style={{flex: 5}}
            />
            <FlatList
            data={this.state.manager.get_lights()}
            renderItem={({item}) => <Light_Ui data={item} manager={this.state.manager} /> }
            keyExtractor={(item:any,index:number) =>item.get_name()+index}//Name
            />
            </View>
        );
    }
}
const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: '#000',
        paddingHorizontal :20,
        paddingTop :90
    },
});
