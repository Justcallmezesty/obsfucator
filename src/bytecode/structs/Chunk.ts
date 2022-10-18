import { MemoryStream } from "../../util/MemoryStream";

export class Chunk {
  byteStream: MemoryStream;

  sourceName: string;
  lineDefined: number;
  lastLineDefined: number;
  upvalueCount: number;
  parameterCount: number;
  varargFlag: number;
  maxStackSize: number;

  constructor(byteStream: MemoryStream) {
    this.byteStream = byteStream;

    this.sourceName = this.byteStream.readString();
    console.log(this.sourceName);

    this.lineDefined = this.byteStream.readInt();
    this.lastLineDefined = this.byteStream.readInt();
    this.upvalueCount = this.byteStream.readInt8();
    this.parameterCount = this.byteStream.readInt8();
    this.varargFlag = this.byteStream.readInt8();
    this.maxStackSize = this.byteStream.readInt8();

    for (let i = 0; i < this.byteStream.readInt(); i++) {
      //TODO: instruction parsing
    }
  }
}