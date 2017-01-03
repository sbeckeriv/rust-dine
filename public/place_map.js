var infowindow = new google.maps.InfoWindow();
var map, markers;
var cluster;
var icons = {};
ZOOM=18;
var latlng = new google.maps.LatLng(place.location[1], place.location[0]);

var myOptions = {
  zoom: parseInt(ZOOM),
  center: latlng,
  mapTypeId: google.maps.MapTypeId.ROADMAP
};
colors = ["green", "yellow", "red", "white"]
for (var ci = 0; ci < colors.length; ci++) {
  icons[colors[ci]] = new google.maps.MarkerImage('/' + colors[ci] + '.png', new google.maps.Size(32, 36), //image size
                                                  new google.maps.Point(0, 0), // The origin
                                                  new google.maps.Point(12, 24) // The anchor
                                                 );
}
function colorFromScore(score) {
    if (!score) {
        return 'white';
    }
    if (score <= 20) {
        return 'green';
    }
    if (score <= 45) {
        return "yellow";
    }
    return 'red';
}
map = new google.maps.Map(document.getElementById("map"), myOptions);
icon = icons[colorFromScore(place["inspection_score"])];
marker =  new google.maps.Marker({
  position: new google.maps.LatLng(place.location[1], place.location[0]),
  map: map,
  title: place["name"] + " -" + place["inspection_score"],
  icon: icon
});

