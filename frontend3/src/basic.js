import React from 'react'
import ReactDOM from 'react-dom'

import Map, {Marker, InfoWindow, GoogleApiWrapper} from 'google-maps-react'
import 'whatwg-fetch'

const Container = React.createClass({
  getInitialState: function() {
    return {
      showingInfoWindow: false,
      activeMarker: {},
      selectedPlace: {place:{}, inspections:[]},
      places: [],
    }
  },

  getPlaces: function(mapProps, map) {
			var bounds = map.getBounds();
			if (bounds) {
					var sw = bounds.getSouthWest();
					var ne = bounds.getNorthEast();
					var url = 'http://localhost:8000/location?sw_lat=' + sw.lat() + "&sw_long=" + sw.lng() + '&ne_lat=' + ne.lat() + "&ne_long=" + ne.lng();
          this.setState({places:
[{"inspections":[],"id":8,"name":"String","program_identifier":"String","description":null,"longitude":-122.3851447207237,"latitude":47.66657874084547},{"inspections":[],"id":7,"name":"String","program_identifier":"String","description":null,"longitude":-122.3851447207237,"latitude":47.66657874084547},{"inspections":[],"id":6,"name":"String","program_identifier":"String","description":null,"longitude":-122.3851447207237,"latitude":47.66657874084547},{"inspections":[],"id":5,"name":"String","program_identifier":"String","description":null,"longitude":-122.3851447207237,"latitude":47.66657874084547},{"inspections":[],"id":4,"name":"String","program_identifier":"String","description":null,"longitude":-122.3851447207237,"latitude":47.66657874084547},{"inspections":[],"id":3,"name":"String","program_identifier":"String","description":null,"longitude":-122.3851447207237,"latitude":47.66657874084547},{"inspections":[],"id":2,"name":"String","program_identifier":"String","description":null,"longitude":-122.3851447207237,"latitude":47.66657874084547},{"inspections":[],"id":1,"name":"String","program_identifier":"String","description":null,"longitude":-122.3851447207237,"latitude":47.66657874084547}]});
return
					fetch(url)
					.then(function(response) {
						return response.json()
					}).then(function(json) {
            this.setState({places: json.results});
					}).catch(function(ex) {
						console.log('parsing failed', ex)
					})
			}else{
				var that=this;
				setTimeout(function() {
					that.getPlaces(mapProps, map)
				},1000)
			}
  },

  onMapMoved: function(props, map) {
    this.getPlaces(props, map);
    const center = props.google.maps.LatLng();
  },

  onMarkerClick: function(props, marker, e) {
    this.setState({
      selectedPlace: props,
      activeMarker: marker,
      showingInfoWindow: true
    });
  },

  onInfoWindowClose: function() {
    this.setState({
      showingInfoWindow: false,
      activeMarker: null
    })
  },

  onMapClicked: function(props) {
    if (this.state.showingInfoWindow) {
      this.setState({
        showingInfoWindow: false,
        activeMarker: null
      })
    }
  },

  render: function() {
    if (!this.props.loaded) {
      return <div>Loading...</div>
    }
    var markers = this.state.places.map((place,index) =>
          <Marker key={place.id} onClick={this.onMarkerClick}
           position={{lat: place.latitude, lng: place.longitude}}
           place={place} name={place.name}  />
    );

    return (
      <Map google={this.props.google}
					onReady={this.getPlaces}
          initialCenter={{lat: 47.6792, lng: -122.3860}}
          style={{width: '100%', height: '100%', position: 'relative'}}
          className={'map'}
          zoom={14}
          containerStyle={{}}
          centerAroundCurrentLocation={true}
          onClick={this.onMapClicked}
          onDragend={this.onMapMoved}>

          {markers}

          <InfoWindow
            marker={this.state.activeMarker}
            visible={this.state.showingInfoWindow}>
              <div>
                <h1>{this.state.selectedPlace.place.name}</h1>
              </div>
          </InfoWindow>
      </Map>
    )
  }
});

export default GoogleApiWrapper({
    apiKey: "AIzaSyA5kAo4Vu1NqICHHrSIV2ZESxdqb4qdceg"
})(Container)
