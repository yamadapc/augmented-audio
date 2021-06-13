import { act, render } from "@testing-library/react";
import { invoke } from "@tauri-apps/api/tauri";
import { useCommand } from "./useCommand";
import { castMock } from "../utils/testUtils";

jest.mock("@tauri-apps/api/tauri", () => ({
  invoke: jest.fn(),
}));

const invokeMock = castMock(invoke);

describe("useCommand", () => {
  it("will call tauri commands on mount", async () => {
    const mockData = { s: true };
    invokeMock.mockReturnValue(Promise.resolve(mockData));

    let onRenderCallArg: any = null;
    const onRender = jest.fn().mockImplementation((r) => {
      onRenderCallArg = r;
    });
    const MockComponent = () => {
      const result = useCommand("command");
      onRender(result);
      return null;
    };
    await act(async () => {
      render(<MockComponent />);
      expect(onRender).toHaveBeenCalledTimes(1);
      expect(onRenderCallArg).toMatchObject({
        loading: true,
        data: null,
        error: null,
      });
      expect(invokeMock).not.toHaveBeenCalled();

      await new Promise((resolve) => setTimeout(resolve, 1));
      expect(invokeMock).toHaveBeenCalled();
      expect(onRender).toHaveBeenCalledTimes(2);
      expect(onRenderCallArg).toMatchObject({
        loading: false,
        data: mockData,
        error: null,
      });
    });
  });

  it("will skip calling tauri commands on mount if skip is true", async () => {
    const mockData = { s: true };
    invokeMock.mockReturnValue(Promise.resolve(mockData));

    const onRender = jest.fn();
    const MockComponent = () => {
      const result = useCommand("command", { skip: true, args: {} });
      onRender(result);
      return null;
    };
    render(<MockComponent />);
    expect(onRender).toHaveBeenCalledTimes(1);
    expect(invokeMock).not.toHaveBeenCalled();
    await new Promise((resolve) => setTimeout(resolve, 1));
    expect(invokeMock).not.toHaveBeenCalled();
    expect(onRender).toHaveBeenCalledTimes(1);
  });
});
