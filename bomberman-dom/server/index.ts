import { PORT } from "./env"
import {httpServer} from "./net"

console.log("listening on "+PORT)
httpServer.listen(PORT)
