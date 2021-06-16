import React, {useState} from 'react';
import {StyleSheet, Text, View, TouchableOpacity } from 'react-native';
import CheckBox from 'react-native-check-box';
import ToggleSwitch from 'toggle-switch-react-native';
//Ok some quick thoughts I don't what state fullness to leak over from the back end to the front end however I am ok with there being state fullness in total.
//OK I have some problems with separation of consurenes right now.

//I want to be able store state in the front end a bit now witch is not what I have done before.

//I think that I will have to move away from props and to state fullness.
//I will then have to be able to update without removing everything.
//I think that will also solve my dirtying problem as I will only be refreshing the light groups not everything.
const Task = (props) =>{
    //toggle state i need to figure out how to propagate change down.
    //I think I might ge able to put the actual views in the state.
    const [light,setlight] = useState(props.data);
    const [selected,setSelected] = useState(false);
    const [color,setColor] = useState(props.data.pattern.Constent.target);

    const getColor = (data) => {
        return data.pattern.Constent.target;
    };

    const toggal= ()=>{
        fetch('http://192.168.1.17:8000/led/'+
              (light.on ? 'off':'on')+'/'+light.name,
              {
                  method: 'PUT'
              })
            .then((response) =>response.json())
            .then((responseJson) => {
                setlight(responseJson);
            })
            .catch((error) => {
                console.error(error);
            });
    };
    const update_color = (props)=>{
        if (selected && props.color && props.color != color){
            setColor(props.color);
            fetch('http://192.168.1.17:8000/led/group/'+light.name,
                  {
                      method: 'PUT',
                      body: JSON.stringify({
                          "Constent": {
                              "target": props.color,
                              "fade_time": {
                                  "secs": 1,
                                  "nanos": 0
                              }
                          }
                      }
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
        <View style={[styles.item,{backgroundColor:color}]}>
          <View style={styles.itemLeft}>
            <CheckBox
              style={styles.square}
              onClick={()=>{
                  setSelected(!selected);
              }}
              isChecked={selected}
            />
            <Text>{props.data.name}</Text>
          </View>
          <ToggleSwitch
            isOn={light.on}
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
