import React from 'react'
import ReactDOM from 'react-dom'
import _ from 'lodash'
import Map, {Marker, InfoWindow, GoogleApiWrapper} from 'google-maps-react'
import 'whatwg-fetch'

const Container = React.createClass({

  getInitialState: function() {
    return {
      showingInfoWindow: false,
      activeInspection: null,
      activeMarker: {},
      selectedPlace: {place:{}, inspections:[]},
      places: [],
    }
  },

  debugJson: function(){
    this.setState({places: [
      { "inspections":[
            { "violations":[
                  { "id":22971,
                     "inspection_id":14648,
                     "kind":"",
                     "points":0,
                     "description":""
                  }
               ],
               "id":14648,
               "place_id":2928,
               "closed":false,
               "inspected_at":"2016-10-03T07:15:00",
               "inspection_type":"consultation/education - field",
               "inspection_score":0
            },
            { "violations":[
                  {
                     "id":22972,
                     "inspection_id":14649,
                     "kind":"",
                     "points":0,
                     "description":""
                  }
               ],
               "id":14649,
               "place_id":2928,
               "closed":false,
               "inspected_at":"2015-12-07T07:15:00",
               "inspection_type":"consultation/education - field",
               "inspection_score":0
            },
            {
               "violations":[
                  {
                     "id":22973,
                     "inspection_id":14650,
                     "kind":"",
                     "points":0,
                     "description":""
                  }
               ],
               "id":14650,
               "place_id":2928,
               "closed":false,
               "inspected_at":"2015-09-21T07:15:00",
               "inspection_type":"return inspection",
               "inspection_score":0
            },
            {
               "violations":[
                  {
                     "id":22974,
                     "inspection_id":14651,
                     "kind":"red",
                     "points":25,
                     "description":"1600 - proper cooling procedure"
                  }, { "id":22975, "inspection_id":14651, "kind":"red", "points":10, "description":"2110 - proper cold holding temperatures (greater than  45 degrees f)" }, { "id":22976, "inspection_id":14651, "kind":"red", "points":5, "description":"0200 - food worker cards current for all food workers; new food workers trained" } ], "id":14651, "place_id":2928, "closed":false, "inspected_at":"2015-09-04T07:15:00", "inspection_type":"routine inspection/field review", "inspection_score":40 }, { "violations":[ { "id":22977, "inspection_id":14652, "kind":"red", "points":15, "description":"2000 - proper reheating procedures for hot holding" }, { "id":22978, "inspection_id":14652, "kind":"blue", "points":5, "description":"4200 - food-contact surfaces maintained, clean, sanitized" }
               ],
               "id":14652,
               "place_id":2928,
               "closed":false,
               "inspected_at":"2015-03-24T07:15:00",
               "inspection_type":"routine inspection/field review",
               "inspection_score":20
            },
            {
               "violations":[
                  {
                     "id":22979,
                     "inspection_id":14653,
                     "kind":"",
                     "points":0,
                     "description":""
                  }
               ],
               "id":14653,
               "place_id":2928,
               "closed":false,
               "inspected_at":"2014-10-27T07:15:00",
               "inspection_type":"consultation/education - field",
               "inspection_score":0
            },
            {
               "violations":[
                  {
                     "id":22980,
                     "inspection_id":14654,
                     "kind":"red",
                     "points":10,
                     "description":"2110 - proper cold holding temperatures (greater than  45 degrees f)"
                  },
                  {
                     "id":22981,
                     "inspection_id":14654,
                     "kind":"red",
                     "points":5,
                     "description":"0200 - food worker cards current for all food workers; new food workers trained"
                  }
               ],
               "id":14654,
               "place_id":2928,
               "closed":false,
               "inspected_at":"2014-07-21T07:15:00",
               "inspection_type":"routine inspection/field review",
               "inspection_score":15
            },
            {
               "violations":[
                  {
                     "id":22982,
                     "inspection_id":14655,
                     "kind":"red",
                     "points":5,
                     "description":"2120 - proper cold holding temperatures ( 42 degrees f to 45 degrees f)"
                  },
                  {
                     "id":22983,
                     "inspection_id":14655,
                     "kind":"blue",
                     "points":5,
                     "description":"3300 - potential food contamination prevented during delivery,  preparation, storage, display"
                  }
               ],
               "id":14655,
               "place_id":2928,
               "closed":false,
               "inspected_at":"2014-03-06T07:15:00",
               "inspection_type":"routine inspection/field review",
               "inspection_score":10
            }
         ],
         "id":2928,
         "name":"el borracho",
         "program_identifier":"el borracho",
         "description":"seating 13-50 - risk category iii",
         "longitude":-122.384274,
         "latitude":47.668233
      },
    ]});
  },

  getPlaces: function(mapProps, map) {
			var bounds = map.getBounds();
			var that=this;
			if (bounds) {
					var sw = bounds.getSouthWest();
					var ne = bounds.getNorthEast();
					var url = 'http://localhost:8000/location?sw_lat=' + sw.lat() + "&sw_long=" + sw.lng() + '&ne_lat=' + ne.lat() + "&ne_long=" + ne.lng();
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

  onDetailsClick: function(e){
    e.preventDefault();
    console.log('The link was clicked.');
    debugger
  },

  onDetailsRemove: function(){
    this.setState({activeInspection: null});
  },

  onMarkerClick: function(props, marker, e) {
    this.setState({
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

  getMarkerIcon: function(inspections){
    var last = this.lastestInspection(inspections);
    if(!last){
      return "white.png";
    }
    if(last.inspection_score==0){
      return "white.png"
    }
    if(last.inspection_score<=20){
      return "green.png"
    }
    if(last.inspection_score<=50){
      return "yellow.png"
    }
    return "red.png"
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
                  return (<tr key={violation.id} style={{'text-align': "left"}} >
                    <td style={{'min-width': '40px'}}>{violation.kind}</td>
                    <td style={{'min-width': '10px'}}>{violation.points}</td>
                    <td>{violation.description}</td>
                  </tr>)
        });
        return (
          <table  style={{'text-align': "left"}} >
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
    if(selectedPlace && !this.state.activeInspection){
      var place = selectedPlace.place;
      if(place){
        var that = this;
        var inspections = null;
        var non_education = this.realInspections(place.inspections);
        if(non_education[0]){
          inspections = non_education.map((inspection) =>{
              var date = new Date(inspection.inspected_at);
              return ([
                <tr key={inspection.id}  style={{'text-align': "left"}} >
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
         <table  style={{'text-align': "left"}} >
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
      }
    }
  },

  render: function() {
    if (!this.props.loaded) {
      return <div>Loading...</div>
    }
    var that = this;
    var markers = this.state.places.map((place,index) =>{
          var icon = this.getMarkerIcon(place.inspections);
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
            onClicked={this.onDetailsClick}
            marker={this.state.activeMarker}
            visible={this.state.showingInfoWindow}>
              <div>
                <h1>{this.state.selectedPlace.name}</h1>
                {this.renderDetails(this.state.selectedPlace)}
              </div>
          </InfoWindow>
      </Map>
    )
  }
});

export default GoogleApiWrapper({
    apiKey: "AIzaSyA5kAo4Vu1NqICHHrSIV2ZESxdqb4qdceg"
})(Container)
