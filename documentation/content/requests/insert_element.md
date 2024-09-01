+++
title = 'Insert_element'
date = 2024-09-01T15:31:39-05:00
draft = false
+++

## Insert a childless HTML element.

|**Request**|*Arguments*|
|---|---|
|**.insert_element**| *element* The element name.|

This is used to insert an HTML element that cannot have children.
Any required attributes for the element are set in the defined strings matching
*element*-*attribute*.

It is recommended to consult
[the MDN web docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes)
about attributes for more details about which attributes are available.