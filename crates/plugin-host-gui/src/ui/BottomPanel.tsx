import { VolumeMeter } from "./VolumeMeter";
import React from "react";
import styled from "styled-components";

const BottomPanelContainer = styled.div({
  display: "flex",
});

const Expander = styled.div({
  flex: 1,
});

const Item = styled.div({
  padding: "5px 5px",
  borderLeft: "solid 1px black",
});

export function BottomPanel() {
  return (
    <BottomPanelContainer>
      <Expander />
      <Item>
        <VolumeMeter />
      </Item>
    </BottomPanelContainer>
  );
}
