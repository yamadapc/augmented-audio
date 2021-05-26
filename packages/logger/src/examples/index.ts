import { LoggerFactory } from "..";

const logger = LoggerFactory.getLogger("Main");

logger.info("hello world");
const child = logger.child("Child", { version: 3 });
child.error("Hey");
child.warn("Ops");
child.debug("Yes");
