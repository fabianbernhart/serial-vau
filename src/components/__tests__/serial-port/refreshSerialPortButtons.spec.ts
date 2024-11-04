// @vitest-environment jsdom

import RefreshSerialPortsButton from "@/components/serial-port/RefreshSerialPortsButton.vue";
import { useAppStore } from "@/stores/app";
import { mount } from "@vue/test-utils";
import { createTestingPinia } from "@pinia/testing";
import { test, describe, vi, expect, beforeEach } from "vitest";

describe("refresh button", () => {
  beforeEach(() => {
    // Set up Pinia for each test
  });

  test("button should refresh serial ports on click", () => {
    const pinia = createTestingPinia({
      createSpy: vi.fn(),
    });

    let wrapper = mount(RefreshSerialPortsButton, {
      plugins: [pinia],
    });

    const serialPortSpy = vi.fn();

    const appStore = useAppStore();

    appStore.getSerialPorts = serialPortSpy;
    console.warn = vi.fn();

    wrapper.find<HTMLButtonElement>("button").trigger("click");

    expect(console.warn).toBeCalled();

    expect(serialPortSpy).toHaveBeenCalled();
  });
});
