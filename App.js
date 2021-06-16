import { StatusBar } from 'expo-status-bar';
import React, {Component, } from 'react';
import { KeyboardAvoidingView, FlatList, StyleSheet, Text, View, TextInput, TouchableOpacity, Keyboard, ScrollView } from 'react-native';
import Task from './components/Task';
import { ColorPicker,fromHsv } from 'react-native-color-picker';
export default class App extends Component {
    state = {
        taskItems: [],
        color: null,
    }
    constructor() {
        super();
        fetch('http://192.168.1.17:8000/led/group', {
            method: 'GET'
        })
            .then((response) =>response.json())
            .then((responseJson) => {
                this.setState({ taskItems:responseJson });
                console.log(responseJson);
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
                    color =>{
                        this.setState({ color:fromHsv(color)+"ff"});
                    }
                }
                style={{flex: 5}}
              />
              <FlatList
                data={this.state.taskItems}
                renderItem={({item}) => <Task data={item} color={this.state.color} /> }
                keyExtractor={(item,index) =>item.name}

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
