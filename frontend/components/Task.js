import React, {useState} from 'react';
import {StyleSheet, Text, View, TouchableOpacity } from 'react-native';
import CheckBox from 'react-native-check-box';
import ToggleSwitch from 'toggle-switch-react-native';
const Task = (props) =>{
    const [light,setlight] = useState(props.data);
    const [selected,setSelected] = useState(false);
    const [color,setColor] = useState(props.data[1]);//.pattern.Constent.target);

    const getColor = (data) => {
        return data[1];//.pattern.Constent.target;
    };

    //This stuff will break.
    const toggal= ()=>{
        var url = 'http://192.168.1.17:8000/'+
            ((light[1]!="#000000ff")? 'off':'on')+'/'+light[0];
        fetch(url,
              {
                  method: 'POST'
              })
            .then((response) =>{
                return response.json();
            })
            .then((responseJson) => {
                console.log("should be"+responseJson);
                setlight(responseJson);
                console.log("is "+light);
            })
            .catch((error) => {
                console.error(error);
            });
    };
    const update_color = (props)=>{
        if (selected && props.color && props.color != color){
            setColor(props.color);
            fetch('http://192.168.1.17:8000/set/'+light[0],//.name,
                  {
                      method: 'Post',
                      body: JSON.stringify(
                          props.color
                          /*
                          {
                          "Constent": {
                              "target": props.color,
                              "fade_time": {
                                  "secs": 1,
                                  "nanos": 0
                              }
                          }
                      }
                          */
                                          )})
                .then((response) =>response.json())
                .then((responseJson) => {
                    setlight(responseJson);
                    console.log(responseJson);
                })
                .catch((error) => {
                    console.error(error);
                });
        }
    };
    update_color(props);
    return (
        <View style={[styles.item,{backgroundColor:light[1]}]}>
          <View style={styles.itemLeft}>
            <CheckBox
              style={styles.square}
              onClick={()=>{
                  setSelected(!selected);
              }}
              isChecked={selected}
            />
            <Text>{props.data[0]/*.name*/}</Text>
          </View>
          <ToggleSwitch
            isOn={light[1]!="#000000ff"}//.on}
            onColor="green"
            offColor="red"
            size="large"
            onToggle={isOn => toggal(props)}
          />
        </View>
    );
};

export default Task;

const styles = StyleSheet.create({
    item: {
        backgroundColor: '#fff',
        padding: 15,
        borderRadius :10,
        flexDirection:'row',
        alignItems:'center',
        justifyContent:'space-between',
        marginBottom : 20,
    },
    itemLeft: {
        flexDirection:'row',
        alignItems:'center',
        flexWrap: 'wrap'
    },
    square: {
        width: 24,
        height: 24,
        backgroundColor: '#66f',
        opacity : .4,
        borderRadius :5,
        marginRight: 15,
    },
    itemText:{
        maxWidth :80,
    },
    circuler:{
        width: 12,
        height: 12,
        backgroundColor: '#66f',
        borderRadius :5,
        borderWidth:2,
    }

});
