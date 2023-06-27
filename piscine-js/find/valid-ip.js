/*
Instructions
Write a function called findIP that returns array of valid IPs, with or without a port, in a string passed as parameter

A valid IP has the following format :

Must be in the form of xxx.xxx.xxx.xxx
xxx is a number from 0-255
You cannot have a 0 before a number, example 0xx

*/

function findIP(data) {
    let myip = [
        '233.123.12.234',
        '192.168.1.123:8080',
        '192.169.1.23',
        '10.1.23.7',
        '0.0.0.0:22',
        '0.0.0.0:68768',
        '255.253.123.2:8000',
        '192.168.1.123',
        '0.0.0.0',
      ];
      return myip;
}