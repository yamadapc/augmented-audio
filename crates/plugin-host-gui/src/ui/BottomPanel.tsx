import {VolumeMeter} from "./VolumeMeter";
import React from "react";
import styled from "styled-components";
import {BORDER_COLOR} from "./constants";
import {TransportControls} from "./TransportControls";

const BottomPanelContainer = styled.div({
  display: "flex",
});

const Expander = styled.div({
  flex: 1,
});

const Item = styled.div({
  padding: "5px 5px",
  borderLeft: `solid 1px ${BORDER_COLOR}`,
  borderRight: `solid 1px ${BORDER_COLOR}`,
});

export function BottomPanel() {
  return (
    <BottomPanelContainer>
      <Expander />
      <Item>
        <TransportControls />
      </Item>
      <Expander />
      <Item>
        <VolumeMeter />
      </Item>
    </BottomPanelContainer>
  );
}
