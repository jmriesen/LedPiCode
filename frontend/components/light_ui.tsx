import React, {useState} from 'react';
import {StyleSheet, Text, View, TouchableOpacity } from 'react-native';
import CheckBox from 'react-native-check-box';
import ToggleSwitch from 'toggle-switch-react-native';

import {Light, black,white} from './light';
const Light_Ui = (props:any) =>{
    const [update_toggle,set_update] = useState(false);
    let light = props.data;

    const update = ()=>{
        set_update(!update_toggle);
    }

    return (
        <View style={[styles.item,{backgroundColor:light.get_color()}]}>
          <View style={styles.itemLeft}>
            <CheckBox
              style={styles.square}
              onClick={()=>{
                  props.manager.select_toggle(light);
                  update();
              }}
                isChecked={props.manager.is_selected(light)}
            />
            <Text>{light.get_name()}</Text>
          </View>
          <ToggleSwitch
            isOn={light.get_color()!="#000000ff"}
            onColor="green"
            offColor="red"
            size="large"
              onToggle={(isOn:boolean) => light.toggle().then(update)}
          />
        </View>
    );
};

export default Light_Ui;

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
