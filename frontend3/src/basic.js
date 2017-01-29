import React from 'react'
import ReactDOM from 'react-dom'
import _ from 'lodash'
import Map, {Marker, InfoWindow, GoogleApiWrapper} from 'google-maps-react'
import { Grid, Row, Col} from 'react-bootstrap';
import 'whatwg-fetch'
import Slider from 'rc-slider'
import 'rc-slider/assets/index.css';

const MapContainer = React.createClass({
  getInitialState: function() {
    return {
      map: null,
      selectedDetails: null,
      showingInfoWindow: false,
      activeInspection: null,
      activeMarker: {},
      selectedPlace: {place:{}, inspections:[]},
      scoreFilter: [0, 300],
      places: [],
    }
  },

  getDetails: function(place){
    if(place){
			var that=this;
      var url = '/inspections?place_id='+place.id;
      if(window.location.host.includes("localhost:3000")){
        url = "http://localhost:8000"+url;
      }
      fetch(url)
      .then(function(response) {
        return response.json()
      }).then(function(json) {
        that.setState({selectedDetails: json.results[0]});
      }).catch(function(ex) {
        console.log('parsing failed', ex)
      })
    }
  },

  onReady: function(mapProps, map){
      this.setState({map: map});
      this.renderAutoComplete(map);
      this.getPlaces(mapProps, map);
  },

  getPlaces: function(mapProps, map) {
      if(!this.state.map){
        return;
      }
			var bounds = this.state.map.getBounds();
			var that=this;
			if (bounds) {
					var sw = bounds.getSouthWest();
					var ne = bounds.getNorthEast();
          var min = that.state.scoreFilter[0];
          var max = that.state.scoreFilter[1];
          if(mapProps.scoreFilter){
              min = mapProps.scoreFilter[0];
              max = mapProps.scoreFilter[1];
          }
					var url = '/location?sw_lat=' + sw.lat() + "&sw_long=" + sw.lng() + '&ne_lat=' +
            ne.lat() + "&ne_long=" + ne.lng()+"&min="+min+"&max="+max;
          if(window.location.host.includes("localhost:3000")){
            url = "http://localhost:8000"+url;
          }
					fetch(url)
					.then(function(response) {
						return response.json()
					}).then(function(json) {
            that.setState({places: json.results});
					}).catch(function(ex) {
						console.log('parsing failed', ex)
					})
			}else{
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
    this.getDetails(props.place);
    this.setState({
      selectedDetails: null,
      selectedPlace: props,
      activeInspection: null,
      activeMarker: marker,
      showingInfoWindow: true
    });
  },

  onInfoWindowClose: function() {
    this.setState({
      showingInfoWindow: false,
      activeInspection: null,
      activeMarker: null
    })
  },

  onMapClicked: function(props) {
    if (this.state.showingInfoWindow) {
      this.setState({
        showingInfoWindow: false,
        activeMarker: null,
        activeInspection: null
      })
    }
  },

  getMarkerIcon: function(place){
    var base = "//s3-us-west-2.amazonaws.com/rustdine/";
    var last = place.most_recent_score;
    if(!last || last<1){
      return base + "white.png";
    }
    if(last<=20){
      return base + "green.png";
    }
    if(last<=50){
      return base + "yellow.png";
    }
    return base + "red.png";
  },

	lastestInspection: function(inspections){
		var filtered= this.realInspections(inspections);
    if(filtered[0]){
      var sorted = _.sortBy(filtered, [function(o) { return o.inspected_at; }]);
      return _.last(sorted);
    }
	},

  realInspections: function(inspections){
    if(inspections){
      return _.filter(inspections, function(o) { return o.inspection_type!="consultation/education - field"; });
    }else{
      return []
    }
  },
  renderInspectionDetails: function(inspection){
      if(inspection.violations[0]){

        var list = inspection.violations.map((violation)=>{
            return (
              <tr key={violation.id} style={{'textAlign': "left"}} >
                <td style={{minWidth: '40px'}}>{violation.kind}</td>
                <td style={{minWidth: '10px'}}>{violation.points}</td>
                <td>{violation.description}</td>
            </tr>
            )
        });

        return (
          <table  style={{'textAlign': "left"}} >
            <thead>
            <tr>
              <th>Kind</th>
              <th>Points</th>
              <th>Description</th>
            </tr>
            </thead>
            <tbody>
              {list}
            </tbody>
          </table>
        );
      }
  },

  renderDetails: function(selectedPlace){
    if(!selectedPlace){
      return (
        <div>
          Loading...
        </div>
      )
    }
    var that = this;
    var inspections = null;
    var non_education = this.realInspections(selectedPlace.inspections);
    if(non_education[0]){
      var non_education_sorted = _.sortBy(non_education, [function(o) { return o.inspected_at; }]).reverse();
      inspections = non_education_sorted.map((inspection) =>{
          var date = new Date(inspection.inspected_at);
          return ([
            <tr key={inspection.id}  style={{'textAlign': "left"}} >
              <td>{date.toLocaleDateString()}</td>
              <td>{inspection.inspection_score}</td>
            </tr>,
            <tr colSpan={2} key={"sub"+inspection.id}>
              <td>
                {that.renderInspectionDetails(inspection)}
              </td>
            </tr>
          ])
        }
      )
    }
    return (
     <table  style={{'textAlign': "left"}} >
        <thead>
          <tr>
            <th>Inspection Date</th>
            <th>Total Score</th>
          </tr>
        </thead>
        <tbody>
          {inspections}
        </tbody>
      </table>
    )
  },

  renderAutoComplete: function(map) {
    const google = this.props.google;
    if (!google || !map ) return;
    const aref = this.refs.autocomplete;
    const node = ReactDOM.findDOMNode(aref);
    var autocomplete = new google.maps.places.Autocomplete(node);
    autocomplete.bindTo('bounds', map);

    autocomplete.addListener('place_changed', () => {
      const place = autocomplete.getPlace();
      if (!place.geometry) {
        return;
      }

      if (place.geometry.viewport) {
        map.fitBounds(place.geometry.viewport);
        map.setZoom(18);
      } else {
        map.setCenter(place.geometry.location);
        map.setZoom(18);
      }
    })
  },

  updateFilter: function(e) {
    this.setState({scoreFilter: e});
    this.getPlaces({scoreFilter: e},{});
  },

  onSubmit: function(e) {
    e.preventDefault();
  },

  render: function() {
    if (!this.props.loaded) {
      return <div>Loading...</div>
    }
    var that = this;
    var markers = this.state.places.map((place,index) =>{
          var icon = this.getMarkerIcon(place);
          var iconProps = {
              url: icon,
              anchor:new that.props.google.maps.Point(12,24),
          };
          return(<Marker key={place.id} onClick={this.onMarkerClick}
           position={{lat: place.latitude, lng: place.longitude}}
           place={place} name={place.name} icon={ iconProps }
           />)
    }
    );

    return (
        <Grid fluid={true}>
          <Row>
            <Col sm={3} >
              <form id='googleAuto'  onSubmit={this.onSubmit}>
                <input
                  ref='autocomplete'
                  type="text"
                  placeholder="Enter a location" />
                <input
                  className='button'
                  type='submit'
                  value='Go' />
              </form>
            </Col>
            <Col sm={4}>
              <Slider range allowCross={false} defaultValue={[0,300]} onAfterChange={this.updateFilter} max={300} />
              <span>Score Filter</span>
            </Col>
          </Row>
          <Row >
            <Col xl={12} >
              <div>
                <Map google={this.props.google}
                    onReady={this.onReady}
                    initialCenter={{lat: 47.6792, lng: -122.3860}}
                    style={{height: '90%'}}
                    className={'map'}
                    zoom={14}
                    containerStyle={{}}
                    centerAroundCurrentLocation={true}
                    onClick={this.onMapClicked}
                    onDragend={this.onMapMoved}
                    onZoom_changed={this.onMapMoved}>

                    {markers}

                    <InfoWindow
                      onClicked={this.onDetailsClick}
                      marker={this.state.activeMarker}
                      visible={this.state.showingInfoWindow}>
                        <div>
                          <h1>{this.state.selectedPlace.name}</h1>
                          {this.renderDetails(this.state.selectedDetails)}
                        </div>
                    </InfoWindow>
                </Map>
              </div>
            </Col>
          </Row>
        </Grid>
    )
  }
});

export default GoogleApiWrapper({
    apiKey: "AIzaSyA5kAo4Vu1NqICHHrSIV2ZESxdqb4qdceg"
})(MapContainer)
