import { instantiate } from "../lib/rs_lib.generated.js";
import { serve } from "https://deno.land/std@0.145.0/http/server.ts";

const { add } = await instantiate();

const total = add(1, 2);
serve((req: Request) => new Response(JSON.stringify(total), {
    status: 200,
    headers: {
        "content-type": "application/json"
    }
}));