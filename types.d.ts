export interface NjdNode {
  surface: string;
  pos: string;
  posGroup1: string;
  posGroup2: string;
  posGroup3: string;
  ctype: string;
  cform: string;
  read: string | null;
  pronunciation: string;
  accent: number;
  moraSize: number;
  chainRule: string;
  chainFlag: boolean | null;
}

export interface AnalysisResult {
  nodes: NjdNode[];
}

export interface InspectResult {
  before: NjdNode[];
  after: NjdNode[];
}

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export class JPreprocess {
  constructor();
  analyze(text: string, preprocess?: boolean): AnalysisResult;
  inspect(text: string): InspectResult;
  normalize(text: string): string;
  fullContext(text: string): string[];
}

export default function init(
  input?: InitInput | Promise<InitInput>
): Promise<InitOutput>;
