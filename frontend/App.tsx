import { StatusBar } from 'expo-status-bar';
import React, {Component, } from 'react';
import { KeyboardAvoidingView, FlatList, StyleSheet, Text, View, TextInput, TouchableOpacity, Keyboard, ScrollView } from 'react-native';
import Light_Ui from './components/light_ui';
import { ColorPicker,fromHsv,} from 'react-native-color-picker';

export default class App extends Component {
    state = {
        taskItems: [],
        color: null,
    }
    constructor(props:any) {
        super(props);
        fetch('http://192.168.1.17:8000/status', {
            method: 'GET'
        })
            .then((response) =>response.json())
            .then((responseJson) => {
                this.setState({ taskItems:responseJson });
                console.log(responseJson);
                console.log(responseJson[0][0]);
            })
            .catch((error) => {
                console.error(error);
            });
    }
    render() {
        return (
            <View style={styles.container}>
                <ColorPicker
                    onColorSelected={
                    (color)=>{
                        //this.setState({ color:fromHsv(color)+"ff"});
                        this.setState({ color:color+"ff"});
                    }
                    }
                    style={{flex: 5}}
                />
                <FlatList
                    data={this.state.taskItems}
                    renderItem={({item}) => <Light_Ui data={item} color={this.state.color} /> }
                    keyExtractor={(item:any,index:number) =>item[1]+index}//Name

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
