// = copyright ====================================================================
// DAW: Flutter UI for a DAW application
// Copyright (C) 2022  Pedro Tacla Yamada
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
// = /copyright ===================================================================
use actix::SystemService;

use audio_processor_graph::NodeType;

use plugin_host_lib::actor_system::ActorSystem;
use plugin_host_lib::audio_io::audio_graph;
use plugin_host_lib::audio_io::audio_graph::{AudioGraphManager, ProcessorSpec};

pub fn audio_node_create_raw(processor: NodeType) -> usize {
    let index = ActorSystem::current().spawn_result(async move {
        let manager = AudioGraphManager::from_registry();
        manager
            .send(audio_graph::CreateAudioNodeMessage {
                processor_spec: ProcessorSpec::RawProcessor { value: processor },
            })
            .await
            .unwrap()
            .unwrap()
            .index()
    });
    index
}
