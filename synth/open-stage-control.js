[
    {
        "label": "KEYB",
        "widgets": [
            {
                "type": "keyboard",
                "id": "keyboard",
                "label": false,
                "left": 0,
                "top": 0,
                "width": "100%",
                "height": "100%",
                "color": "auto",
                "css": "",
                "precision": 1,
                "address": "/note",
                "preArgs": [],
                "target": [],
                "keys": 24,
                "start": 60,
                "traversing": true,
                "on": 1,
                "off": 0,
                "split": false
            }
        ]
    },
    {
        "label": "OSC",
        "tabs": [
            {
                "label": "1",
                "widgets": [
                    {
                        "type": "knob",
                        "id": "amp",
                        "linkId": "",
                        "label": "Amplitude",
                        "unit": "",
                        "left": 0,
                        "top": 120,
                        "width": 90,
                        "height": 120,
                        "noPip": false,
                        "compact": false,
                        "color": "auto",
                        "css": "",
                        "snap": false,
                        "spring": false,
                        "range": {
                            "min": -100,
                            "max": 100
                        },
                        "origin": 0,
                        "value": 0.8,
                        "logScale": false,
                        "precision": 2,
                        "address": "/osc/amp",
                        "preArgs": [
                            {
                                "type": "i",
                                "value": 1
                            }
                        ],
                        "target": [],
                        "angle": 270
                    },
                    {
                        "type": "multifader",
                        "id": "freq",
                        "label": "Modulation",
                        "left": 0,
                        "top": 240,
                        "width": "100%",
                        "height": 200,
                        "color": "auto",
                        "css": "",
                        "address": "/fm",
                        "preArgs": [
                            {
                                "type": "i",
                                "value": 1
                            }
                        ],
                        "origin": 0,
                        "strips": 8,
                        "start": 1,
                        "unit": "",
                        "alignRight": false,
                        "horizontal": false,
                        "noPip": false,
                        "compact": false,
                        "traversing": true,
                        "snap": true,
                        "range": {
                            "min": -1,
                            "max": 1
                        },
                        "value": "",
                        "logScale": false,
                        "precision": 2,
                        "meter": false,
                        "split": false,
                        "target": []
                    },
                    {
                        "type": "knob",
                        "id": "freq",
                        "linkId": "",
                        "label": "Frequency",
                        "unit": "",
                        "left": 0,
                        "top": 0,
                        "width": 90,
                        "height": 120,
                        "noPip": false,
                        "compact": false,
                        "color": "auto",
                        "css": "",
                        "snap": false,
                        "spring": false,
                        "range": {
                            "min": 0,
                            "max": 14000
                        },
                        "origin": "auto",
                        "value": 0,
                        "logScale": false,
                        "precision": 2,
                        "address": "/osc/freq",
                        "preArgs": [
                            {
                                "type": "i",
                                "value": 1
                            }
                        ],
                        "target": [],
                        "angle": 270
                    },
                    {
                        "type": "toggle",
                        "top": 160,
                        "left": 270,
                        "id": "fixed-freq",
                        "linkId": "",
                        "label": "Fixed Freq",
                        "width": 90,
                        "height": 40,
                        "color": "auto",
                        "css": "",
                        "on": 1,
                        "off": 0,
                        "value": 0,
                        "precision": 0,
                        "address": "/osc/fixed-freq",
                        "preArgs": [
                            {
                                "type": "i",
                                "value": 1
                            }
                        ],
                        "target": []
                    },
                    {
                        "type": "toggle",
                        "top": 120,
                        "left": 270,
                        "id": "enabled",
                        "linkId": "",
                        "label": "Enabled",
                        "width": 90,
                        "height": 40,
                        "color": "auto",
                        "css": "",
                        "on": 1,
                        "off": 0,
                        "value": 0,
                        "precision": 0,
                        "address": "/osc/enabled",
                        "preArgs": [
                            {
                                "type": "i",
                                "value": 1
                            }
                        ],
                        "target": []
                    },
                    {
                        "type": "knob",
                        "id": "ratio",
                        "linkId": "",
                        "label": "Ratio",
                        "unit": "",
                        "left": 90,
                        "top": 0,
                        "width": 90,
                        "height": 120,
                        "noPip": false,
                        "compact": false,
                        "color": "auto",
                        "css": "",
                        "snap": false,
                        "spring": false,
                        "range": {
                            "min": -8,
                            "max": 8
                        },
                        "origin": 0,
                        "value": 0,
                        "logScale": false,
                        "precision": 1,
                        "address": "/osc/octaves",
                        "preArgs": [
                            {
                                "type": "i",
                                "value": 1
                            }
                        ],
                        "target": [],
                        "angle": 270
                    },
                    {
                        "type": "knob",
                        "id": "pitch",
                        "linkId": "",
                        "label": "Pitch",
                        "unit": "",
                        "left": 180,
                        "top": 0,
                        "width": 90,
                        "height": 120,
                        "noPip": false,
                        "compact": false,
                        "color": "auto",
                        "css": "",
                        "snap": false,
                        "spring": false,
                        "range": {
                            "min": -12,
                            "max": 12
                        },
                        "origin": 0,
                        "value": 0,
                        "logScale": false,
                        "precision": 1,
                        "address": "/osc/semitones",
                        "preArgs": [
                            {
                                "type": "i",
                                "value": 1
                            }
                        ],
                        "target": [],
                        "angle": 270
                    },
                    {
                        "type": "multitoggle",
                        "top": 440,
                        "left": 0,
                        "id": "fm-enabled",
                        "width": "100%",
                        "height": 40,
                        "color": "auto",
                        "css": "",
                        "label": false,
                        "matrix": [
                            8,
                            1
                        ],
                        "start": 1,
                        "traversing": true,
                        "on": 1,
                        "off": 0,
                        "value": "",
                        "precision": 2,
                        "address": "/osc/fm/enabled",
                        "preArgs": [
                            {
                                "type": "i",
                                "value": 1
                            }
                        ],
                        "split": false,
                        "target": []
                    },
                    {
                        "type": "knob",
                        "id": "mix",
                        "linkId": "",
                        "label": "Mix",
                        "unit": "",
                        "left": 90,
                        "top": 120,
                        "width": 90,
                        "height": 120,
                        "noPip": false,
                        "compact": false,
                        "color": "auto",
                        "css": "",
                        "snap": false,
                        "spring": false,
                        "range": {
                            "min": 0,
                            "max": 1
                        },
                        "origin": 0,
                        "value": 0,
                        "logScale": false,
                        "precision": 2,
                        "address": "/osc/level",
                        "preArgs": [
                            {
                                "type": "i",
                                "value": 1
                            }
                        ],
                        "target": [],
                        "angle": 270
                    },
                    {
                        "type": "knob",
                        "id": "pan",
                        "linkId": "",
                        "label": "Pan",
                        "unit": "",
                        "left": 180,
                        "top": 120,
                        "width": 90,
                        "height": 120,
                        "noPip": false,
                        "compact": false,
                        "color": "auto",
                        "css": "",
                        "snap": false,
                        "spring": false,
                        "range": {
                            "min": -1,
                            "max": 1
                        },
                        "origin": 0,
                        "value": 0,
                        "logScale": false,
                        "precision": 2,
                        "address": "/osc/pan",
                        "preArgs": [
                            {
                                "type": "i",
                                "value": 1
                            }
                        ],
                        "target": [],
                        "angle": 270
                    },
                    {
                        "type": "knob",
                        "id": "phase",
                        "linkId": "",
                        "label": "Phase",
                        "unit": "",
                        "left": 270,
                        "top": 0,
                        "width": 90,
                        "height": 120,
                        "noPip": false,
                        "compact": false,
                        "color": "auto",
                        "css": "",
                        "snap": false,
                        "spring": false,
                        "range": {
                            "min": 0,
                            "max": {
                                "2π": 6.2832
                            }
                        },
                        "origin": 0,
                        "value": 0,
                        "logScale": false,
                        "precision": 2,
                        "address": "/osc/phase",
                        "preArgs": [
                            {
                                "type": "i",
                                "value": 1
                            }
                        ],
                        "target": [],
                        "angle": 270
                    },
                    {
                        "type": "toggle",
                        "top": 200,
                        "left": 270,
                        "id": "free-phase",
                        "linkId": "",
                        "label": "Free Phase",
                        "width": 90,
                        "height": 40,
                        "color": "auto",
                        "css": "",
                        "on": 1,
                        "off": 0,
                        "value": 0,
                        "precision": 0,
                        "address": "/osc/free-phase",
                        "preArgs": [
                            {
                                "type": "i",
                                "value": 1
                            }
                        ],
                        "target": []
                    },
                    {
                        "type": "keyboard",
                        "id": "keyboard",
                        "label": false,
                        "left": 360,
                        "top": 0,
                        "width": 300,
                        "height": 200,
                        "color": "auto",
                        "css": "",
                        "precision": 1,
                        "address": "/note",
                        "preArgs": [],
                        "target": [],
                        "keys": 18,
                        "start": 60,
                        "traversing": true,
                        "on": 1,
                        "off": 0,
                        "split": false
                    },
                    {
                        "type": "push",
                        "top": 200,
                        "left": 360,
                        "id": "Sync",
                        "linkId": "",
                        "label": "Sync",
                        "width": 90,
                        "height": 40,
                        "color": "auto",
                        "css": "",
                        "on": 1,
                        "off": 0,
                        "norelease": false,
                        "precision": 0,
                        "address": "/sync",
                        "preArgs": [],
                        "target": []
                    }
                ]
            },
            {
                "label": "2"
            },
            {
                "label": "3"
            },
            {
                "label": "4"
            },
            {
                "label": "5"
            },
            {
                "label": "6"
            },
            {
                "label": "7"
            },
            {
                "label": "8"
            }
        ],
        "widgets": []
    },
    {
        "label": "ENV"
    },
    {
        "label": "FX"
    }
]