function setSlider(color) {
    var top = 300, bottom = 0;
    if (color === 'white') {
        top = 0; bottom = 0;
    }
    if (color === 'red') {
        top = 300; bottom = 46;
    }
    if (color === 'yellow') {
        top = 45; bottom = 21;
    }
    if (color === 'green') {
        top = 20; bottom = 1;
    }
    _gaq.push(['_trackEvent', 'slider', 'pin_set',  color]);

    $("#slider-range").slider("option", "values", [bottom, top]);
    $("#amount").val("" + bottom + " - " + top);

}
var map, markers;
var cluster;
var icons = {};
document.addEventListener("orientationChanged", updateOrientation, false);
function updateOrientation(e) {
// resize window here
    wrapperH = window.innerHeight;
    document.getElementById('map_canvas').style.height = wrapperH + 'px';
}

function initMap(start_lat, start_long) {
  debugger
    var latlng = new google.maps.LatLng(start_lat, start_long);

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

    map = new google.maps.Map(document.getElementById("map_canvas"), myOptions);
    google.maps.event.addListener(map, 'zoom_changed', function () {
        _gaq.push(['_trackEvent', 'map_changed', 'zoom',  map.getZoom()]);

        // do with some kinda load queue
        setTimeout(getDataAndSetMarkers, 3000);
    });
    google.maps.event.addListener(map, 'dragend', function () {
        var bounds = map.getBounds();
        if (bounds) {
          var center = bounds.getCenter();
          _gaq.push(['_trackEvent', 'map_changed', 'dragged',  'lat=' + center.lat() + "&long=" + center.lng()]);
        }
        // do with some kinda load queue
        setTimeout(getDataAndSetMarkers, 3000);
    });
    google.maps.event.addListener(map, 'tilesloaded', function () {
        // do with some kinda load queue
        setTimeout(getDataAndSetMarkers, 50);
    });
    var input = document.getElementById('searchTextField');
    var autocomplete = new google.maps.places.Autocomplete(input);

    autocomplete.bindTo('bounds', map);
    google.maps.event.addListener(autocomplete, 'place_changed', function () {
        infowindow.close();
        var place = autocomplete.getPlace();
        if (place.geometry.viewport) {
            map.fitBounds(place.geometry.viewport);
        } else {
            map.setCenter(place.geometry.location);
            if(place.types.lastIndexOf("restaurant")){
              map.setZoom(20);
            }else{
              map.setZoom(18);
            }
        }
        debugger
        name = autocomplete.getPlace().formatted_address;
    _gaq.push(['_trackEvent', 'autocomplete', 'selected',  name]);

        getDataAndSetMarkers(place.name);
    })


}

function clearAndUpdate() {
    for (i in markers) {
        markers[i].setMap(null);
    }
    MAX_SCORE = $("#slider-range").slider("values", 1);
    MIN_SCORE = $("#slider-range").slider("values", 0);
    markers = {};
    getDataAndSetMarkers();
};

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

function alertCurrentUrl() {
    var url = window.location.host;
    var bounds = map.getBounds();
    if (bounds) {
        var center = bounds.getCenter();
        url = url + '/?max=' + MAX_SCORE + '&min=' + MIN_SCORE + '&lat=' + center.lat() + "&long=" + center.lng() + "&zoom=" + map.getZoom();
        alert(url);
    } else {
        alert("Map Code FAIL. Please try again.");
    }
}
function changeCurrentUrl() {
    var url = window.location.host;
    var bounds = map.getBounds();
    if (bounds) {
        var center = bounds.getCenter();
        url = url + '/?max=' + MAX_SCORE + '&min=' + MIN_SCORE + '&lat=' + center.lat() + "&long=" + center.lng() + "&zoom=" + map.getZoom();
       // debugger
        $("#map_link").text(url);
    } else {
        alert("Map Code FAIL. Please try again.");
    }
}
markers = {};

function getDataAndSetMarkers(name) {
    //get map bounds
    //debugger
    $("#map_link").text("");

    var bounds = map.getBounds();
    if (bounds) {
        // get data
        var sw = bounds.getSouthWest();
        var ne = bounds.getNorthEast();
        //delete markers?
        $.getJSON('/location?max=' + MAX_SCORE + '&min=' + MIN_SCORE + '&sw_lat=' + sw.lat() + "&sw_long=" + sw.lng() + '&ne_lat=' + ne.lat() + "&ne_long=" + ne.lng(), function (data) {
            var items = [];
            for (i = 0; i < data.length; i++) {
                // d is data[index] and i will be de for the event
                d = data[i]
                current_marker = markers[[d.location[1], d.location[0]]]
                if (!current_marker) {
                    //should check for a marker first since we are not deleting.
                    icon = icons[colorFromScore(d["inspection_score"])];
                    marker = new google.maps.Marker({
                        position: new google.maps.LatLng(d.location[1], d.location[0]),
                        map: map,
                        title: d["name"] + " -" + d["inspection_score"],
                        icon: icon
                    });
                    markers[[d.location[1], d.location[0]]] = marker;
                    if (CLUSTER_MARKERS === "true") {
                    }
                    google.maps.event.addListener(marker, 'click', (function (marker, i) {
                        return function () {
                            _gaq.push(['_trackEvent', 'marker', 'view',  d["name"] + " -" + d["inspection_score"]]);
                            var violations = ""
                            if (i.violations) {
                                for (var x = 0; x < i.violations.length; x++) {
                                    if (i.violations[x].type) {
                                        violations = violations + "<p style='color:" + i.violations[x].type + "'>" + i.violations[x].points + " - " + i.violations[x].description + "</p>";
                                    }
                                }
                            }
                            var chart_values="";
                            if (i.inspection_scores && i.inspection_scores.length > 0) {
                                for (var x = 0; x < i.inspection_scores.length; x++) {
                                    chart_values += i.inspection_scores[x] + ","
                                }
                                chart_values= chart_values.substring(0, chart_values.length - 1);
                            }
                            var content = "<b>" + i["name"] + "</b><p>" + i["inspection_type"] + " on " + i["inspected_at"] + "</p><p>Total Score:" + i["inspection_score"] + "</p>"
                            if (chart_values && chart_values!="") {
                                content = content + "Historic inspections: <span  class='sparklines'>"+chart_values+"</span> "

                            }
                            content = content + violations;
                            //var opts = new google.maps.InfoWindowOptions();
                            mwidth=null;
                            if(MOBILE){
                              mwidth=50;
                            }
                            infowindow.setOptions({maxWidth:mwidth,disableAutoPan:false});
                            infowindow.setContent(content);
                            infowindow.open(map, marker);
                            $('.sparklines').sparkline('html',{valueSpots:{":20": 'green', "21\:" : 'red'},width:'60px'} );
                        }
                    })(marker, d));
                    if(name && typeof(name)!="number" && marker && d){
                      debugger
                      if(d["name"].toLowerCase() == name.toLowerCase()){
                        debugger
                      }
                    }
                }
            }
        });
        return true;
    } else {
        return false
    }
};
$(function () {
    $("#slider-range").slider({
        range: true,
        min: 0,
        max: 300,
        values: [MIN_SCORE, MAX_SCORE],
        change: function (event, ui) {
          _gaq.push(['_trackEvent', 'slider', 'change',  "" + ui.values[0] + "-" + ui.values[1]]);
          clearAndUpdate()
        },
        slide: function (event, ui) {
            $("#amount").val("" + ui.values[0] + " - " + ui.values[1]);
        }
    });
    $("#amount").val("" + $("#slider-range").slider("values", 0) + " - " + $("#slider-range").slider("values", 1));
});

function initialize() {
    var inited_map = false
var window.infowindow = new google.maps.InfoWindow();
    //www.dinegerous.com/?max=70&min=0&lat=&long=&zoom=17
    var start_long = -122.3835058910156;
    var start_lat = 47.66723262354798;
    if (START_LAT.length > 0 && START_LONG.length > 0) {
        start_long = parseFloat(START_LONG);
        start_lat = parseFloat(START_LAT);
        initMap(start_lat, start_long)
        inited_map = true
    } else if (MOBILE  && navigator.geolocation) {
        navigator.geolocation.getCurrentPosition(

        function (position) {
            initMap(position.coords.latitude, position.coords.longitude);
        },
        // next function is the error callback
        function (error) {
            start_long = -122.318127;
            start_lat = 47.614342;
            initMap(start_lat, start_long);
        });
    } else {
        inited_map = true
        initMap(start_lat, start_long);
    }
    if (!inited_map) {
        initMap(start_lat, start_long);
    }
}
