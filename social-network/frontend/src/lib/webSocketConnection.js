import ioClient from 'socket.io-client';

const socket = ioClient('/');

export const io = socket;
