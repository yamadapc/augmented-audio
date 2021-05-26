import { mount } from "enzyme";
import React from "react";

import { useLogger, wrapWithLogger } from "./index";
import { Logger } from "./logger";
import { LoggerContext } from "./logger-react-context/LoggerContext";

describe("LoggerContext and hooks", () => {
  it("can not provide a root logger", () => {
    const onRender = jest.fn();
    const rootLogger = new Logger({ name: "root", context: {}, parent: null });

    function MockComponent() {
      const logger = useLogger("child");
      onRender(logger);
      return <div>Here</div>;
    }

    const el = mount(<MockComponent />);
    expect(el.contains("Here")).toBeTruthy();
    expect(onRender).toHaveBeenCalledTimes(1);
    const childLogger = onRender.mock.calls[0][0];
    expect(childLogger.parent).toMatchObject({
      name: "app",
    });
    expect(childLogger.name).toEqual("app>child");
  });

  it("can provide a root logger", () => {
    const onRender = jest.fn();
    const rootLogger = new Logger({ name: "root", context: {}, parent: null });

    function MockComponent() {
      const logger = useLogger("child");
      onRender(logger);
      return <div>Here</div>;
    }

    const el = mount(
      <LoggerContext.Provider value={rootLogger}>
        <MockComponent />
      </LoggerContext.Provider>
    );
    expect(el.contains("Here")).toBeTruthy();
    expect(onRender).toHaveBeenCalledTimes(1);
    const childLogger = onRender.mock.calls[0][0];
    expect(childLogger.parent).toEqual(rootLogger);
    expect(childLogger.name).toEqual("root>child");
  });

  it("provides a `wrapWithLogger` utility that's similar", () => {
    const onRender = jest.fn();
    const rootLogger = new Logger({ name: "root", context: {}, parent: null });

    function MockComponent() {
      const logger = useLogger("child");
      onRender(logger);
      return <div>Here</div>;
    }

    const el = mount(wrapWithLogger(rootLogger, <MockComponent />));
    expect(el.contains("Here")).toBeTruthy();
    expect(onRender).toHaveBeenCalledTimes(1);
    const childLogger = onRender.mock.calls[0][0];
    expect(childLogger.parent).toEqual(rootLogger);
    expect(childLogger.name).toEqual("root>child");
  });
});
