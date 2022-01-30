// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#version 450

layout(location = 0) in vec3 Vertex_Position;

layout(set = 0, binding = 0) uniform CameraViewProj {
  mat4 ViewProj;
  mat4 View;
  mat4 InverseView;
  mat4 Projection;
  vec3 WorldPosition;
  float near;
  float far;
  float width;
  float height;
};

layout(set = 2, binding = 0) uniform Mesh {
  mat4 Model;
  mat4 InverseTransposeModel;
  uint flags;
};


layout(location = 0) out vec3 TexCoords;
layout(location = 1) out float depth;

void main() {
  // ViewProj is Proj * inverse(View). We want to get Proj * inverse(untranslatedView). However,
  // the only bindings available are ProjView and View. So we first get untranslatedView by
  // removing the translation from the View matrix (by clearing the last column), then multiply
  // ProjView * View to undo the earlier Proj * inverse(View). Then we multiply by the
  // untranslated view to get a projection matrix that has the camera's rotation but not position.
  // mat4 untranslatedView = InverseView;
  // untranslatedView[3] = vec4(0.0, 0.0, 0.0, 1.0);

  // mat4 untranslatedProj = Projection * untranslatedView;

  // // We allow rotating the skybox, but not translating (since we need the position to match the
  // // zeroed camera position). To do that, remove the translation from the model matrix.
  // mat4 untranslatedModel = Model;
  // untranslatedModel[3] = vec4(0.0, 0.0, 0.0, 1.0);

  // vec4 pos = untranslatedProj * untranslatedModel * vec4(Vertex_Position, 1.0);

  // // We allow rotating the skybox, but not translating (since we need the position to match the
  // // zeroed camera position). To do that, remove the translation from the model matrix.
  vec4 pos = ViewProj * Model * vec4(Vertex_Position + WorldPosition, 1.0);

  // Use w as z to force the point as far back a possible for depth testing purposes. This makes
  // sure it never draws in front of anything else.
  gl_Position = vec4(pos.xy, 1.0 / (far + 10), pos.w);
  // gl_Position = vec4(pos.xy, pos.w, pos.w);
  // depth = pos.w;

  // Since we're sampling a cubemap, texcoords is just the vertex coordinate.
  TexCoords = Vertex_Position;
}
