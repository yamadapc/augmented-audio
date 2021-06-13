import { ParameterState } from "./ParameterState";

describe("ParameterState", () => {
  it("the value can be set and retrieved", () => {
    const state = new ParameterState(0);
    expect(state.value).toEqual(0);
    state.value = 10;
    expect(state.value).toEqual(10);
  });
});
