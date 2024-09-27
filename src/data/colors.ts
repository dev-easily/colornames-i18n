
import {data, headers} from "./colornames.i18n";

export interface Color {
    hex: string;
    rgb: string;
    hsl: string;
    en?: string;
    locale?: string;
    backups?: string;
    demo?: any;
    names: Record<string, string[]>
}

export default {data: data as Color[], headers};