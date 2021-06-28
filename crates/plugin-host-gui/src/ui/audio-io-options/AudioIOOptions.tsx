import styled from "styled-components";
import {BORDER_COLOR, MEDIUM_GRAY} from "../constants";
import {Header} from "./Header";
import {AudioIOOptionsForm} from "./AudioIOOptionsForm";
import React from "react";
import {Container} from "../Container";

const AudioIOContainer = styled.div({
  borderBottom: `solid 1px ${BORDER_COLOR}`,
});
const HeaderContainer = styled(Container)({
  backgroundColor: MEDIUM_GRAY,
  borderBottom: `solid 1px ${BORDER_COLOR}`,
});

export function AudioIOOptions() {
  return (
    <AudioIOContainer>
      <HeaderContainer>
        <Header>Audio IO</Header>
      </HeaderContainer>
      <AudioIOOptionsForm />
    </AudioIOContainer>
  );
}
