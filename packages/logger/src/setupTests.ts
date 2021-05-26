// jest-dom adds custom jest matchers for asserting on DOM nodes.
// allows you to do things like:
// expect(element).toHaveTextContent(/react/i)
// learn more: https://github.com/testing-library/jest-dom
// import "@testing-library/jest-dom/extend-expect";

// @ts-ignore
import Enzyme from "enzyme";
// @ts-ignore
import Adapter from "enzyme-adapter-react-16";

Enzyme.configure({ adapter: new Adapter() });
