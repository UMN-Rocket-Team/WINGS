import { beforeEach, describe, expect, it, vi } from "vitest";

import App from "@/App";

const render = () => {
    return <App />;
};

describe("App", () => {
    it("renders application", async () => {
        expect(render).toMatchSnapshot();
    });
});