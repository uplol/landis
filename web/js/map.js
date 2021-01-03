/// <reference path="../../typings/index.d.ts" />
var map = L.map('map', {
    crs: L.CRS.Simple,
    zoomDelta: 0.25,
    zoomSnap: 0,
    noWrap: true,
});

var mcTiles = L.tileLayer('/tiles/{z}/{x}/{y}/tile.png', {
    attribution: 'Uplol',
    minNativeZoom: 3,
    maxNativeZoom: 3,
    minZoom: 0,
    maxZoom: 5,
    tileSize: 512,
    noWrap: true,
}).addTo(map);

var CoordViewer = L.Control.extend({
    onAdd: () => {
        var container = L.DomUtil.create('div');
        var gauge = L.DomUtil.create('coords');
        container.style.background = 'rgba(255,255,255,1)';
        container.style.textAlign = 'right';

        map.on('mousemove', event => {
            var coords = coord = map.project(event.latlng, 3);
            gauge.innerHTML = 'Coords: ' + Math.round(coords.x) + ", " + Math.round(coords.y);
        })
        container.appendChild(gauge);

        return container;
    }
});

(new CoordViewer).addTo(map);
map.setView([0, 0], 3);
