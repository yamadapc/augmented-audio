import { render } from "@testing-library/react";
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

    const el = render(<MockComponent />);
    expect(el.queryByText("Here")).not.toBeNull();
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

    const el = render(
      <LoggerContext.Provider value={rootLogger}>
        <MockComponent />
      </LoggerContext.Provider>
    );
    expect(el.queryByText("Here")).not.toBeNull();
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

    const el = render(wrapWithLogger(rootLogger, <MockComponent />));
    expect(el.queryByText("Here")).not.toBeNull();
    expect(onRender).toHaveBeenCalledTimes(1);
    const childLogger = onRender.mock.calls[0][0];
    expect(childLogger.parent).toEqual(rootLogger);
    expect(childLogger.name).toEqual("root>child");
  });
});
