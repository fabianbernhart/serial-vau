import { useListener } from "@/api/listener";
import { describe, it, vi, expect } from "vitest";
import { listen } from "@tauri-apps/api/event";
import { useTheme } from "vuetify";
import { afterEach, beforeEach } from "node:test";

describe("uesListener composable", () => {
  beforeEach(() => {
    vi.mock("@/stores/app", () => {
      const mockAppStore = {
        managedSerialPorts: "mockPorts",
        addPacket: vi.fn(),
        getSerialPorts: vi.fn(),
      };
      return { useAppStore: vi.fn(() => mockAppStore) };
    });

    vi.mock("vuetify", () => ({
      useTheme: vi.fn().mockReturnValue({
        global: {
          name: {
            value: "light",
          },
        },
      }),
    }));

    vi.mock("@tauri-apps/api/event", () => ({
      listen: vi.fn().mockResolvedValue(vi.fn()),
    }));
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  it("should call functions from app store", async () => {
    const mockAppStore = {
      managedSerialPorts: "mockPorts",
      addPacket: vi.fn(),
      getSerialPorts: vi.fn(),
    };

    const { setupListeners, cleanupListeners } = useListener(
      mockAppStore as any
    );

    await setupListeners();

    expect(listen).toHaveBeenCalledTimes(3);
    expect(listen).toHaveBeenCalledWith(
      "tauri://theme-changed",
      expect.any(Function)
    );
    expect(listen).toHaveBeenCalledWith(
      "serial_ports_event",
      expect.any(Function)
    );
    expect(listen).toHaveBeenCalledWith(
      "serial_packet_event",
      expect.any(Function)
    );

    expect(mockAppStore.getSerialPorts).toHaveBeenCalled();

    cleanupListeners();

    expect(mockAppStore.addPacket).not.toHaveBeenCalled();
  });
});
