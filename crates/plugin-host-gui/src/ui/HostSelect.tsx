import styled from "styled-components";
import Select, { GroupTypeBase } from "react-select";
import React from "react";
import { StylesConfig } from "react-select/src/styles";

export const SelectWrapper = styled.div({
  userSelect: "none",
  alignItems: "center",
  display: "flex",
  marginBottom: 10,
});

export const SelectLabel = styled.div({
  userSelect: "none",
  minWidth: 150,
  textAlign: "right",
  fontWeight: "bold",
  marginRight: 10,
});

const SelectExpand = styled.div({
  flex: 1,
});

type OptionType<T> = { label: string; value: T };

interface Props<T> {
  label: string;
  options: OptionType<T>[];
}

function getReactSelectStyles<T>(): StylesConfig<
  OptionType<T>,
  false,
  GroupTypeBase<OptionType<T>>
> {
  return {
    option: (base) => ({
      ...base,
      background: "#232323",
      border: "solid 1px #232323",
      borderBottom: "solid 1px black",
      "&:hover": {
        background: "#313030",
        border: "solid 1px #00e1ff",
      },
    }),
    control: (base, props) => ({
      ...base,
      background: "#232323",
      borderRadius: 0,
      border: props.menuIsOpen ? "solid 1px #00e1ff" : "solid 1px black",
      outline: "none",
      "&hover": {
        border: props.menuIsOpen ? "solid 1px #00e1ff" : "solid 1px black",
      },
    }),
    menu: (base) => ({
      ...base,
      borderRadius: 0,
      marginTop: 2,
      padding: 0,
    }),
    menuList: (base) => ({
      ...base,
      margin: 0,
      padding: 0,
    }),
    valueContainer: (base) => ({
      ...base,
    }),
    singleValue: (base) => ({
      ...base,
      color: "white",
      margin: 0,
      padding: 0,
      "&:focus": {
        border: "solid 1px #00e1ffff",
      },
    }),
  };
}

export function HostSelect<T>(props: Props<T>) {
  const reactSelectStyles = getReactSelectStyles();
  return (
    <SelectWrapper>
      <SelectLabel>{props.label}</SelectLabel>
      <SelectExpand>
        <Select
          value={props.options[0]}
          options={props.options}
          styles={reactSelectStyles}
        />
      </SelectExpand>
    </SelectWrapper>
  );
}
