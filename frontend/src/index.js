import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import SimpleMapPage  from './simple';
import './index.css';
import './main.css';
import { withGoogleMap, GoogleMap,
  Marker, } from "react-google-maps";
import _ from "lodash";
import {
  Component,
} from "react";


const GettingStartedGoogleMap = withGoogleMap(props => (
  <GoogleMap
    ref={props.onMapLoad}
    defaultZoom={3}
    defaultCenter={{ lat: -25.363882, lng: 131.044922 }}
    onClick={props.onMapClick}
  >
  </GoogleMap>
));
// Then, render it:
ReactDOM.render(
< SimpleMapPage/>,
  document.getElementById('root'));
/*
ReactDOM.render(
  <GettingStartedGoogleMap
    containerElement={
      <div style={{ height: `100%` }} />
    }
    mapElement={
      <div style={{ height: `100%` }} />
    }
    onMapLoad={_.noop}
    onMapClick={_.noop}
    onMarkerRightClick={_.noop}
  />,
  document.getElementById('root')
);
*/

